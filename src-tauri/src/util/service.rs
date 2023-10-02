//! Utility to create concurrent service tasks/threads with message passing and
//! restart capabilities.
//! 
//! Really just a unified interface for a relatively simple pattern, which I was
//! annoyed at having to recreate multiple times.

use async_trait::async_trait;
use futures::never::Never;
use std::{
    sync::{mpsc as std_mpsc, Arc, Mutex as StdMutex},
    thread
};
use tokio::{
    sync::{mpsc as tokio_mpsc, Mutex as TokioMutex},
    task
};

/// Handle for a service that runs asynchronously, on a tokio task.
pub struct TaskServiceHandle<I, O> {
    inbox: Arc<TokioMutex<tokio_mpsc::UnboundedReceiver<I>>>,
    outbox: Arc<TokioMutex<tokio_mpsc::UnboundedSender<O>>>,
    handle: task::JoinHandle<Never>,
}

/// A service that runs asynchronously on a tokio task.
/// 
/// `I` and `O` refer to the incoming/outgoing types for messages sent to the
/// handle. I.e., this type consumes `O`s and produces `I`s.
#[async_trait]
pub trait TaskService<I: Send + 'static, O: Send + 'static> where Self: Sized + Send + 'static {
    /// Run the service. This is also the way that the service restarts.
    async fn run(&mut self, inbox: tokio_mpsc::UnboundedReceiver<O>, outbox: tokio_mpsc::UnboundedSender<I>);

    /// Optional restart callback, to help keep code cleaner. This will not get
    /// called the first time the service starts.
    async fn on_restart(&mut self) {}

    /// Start the service, consuming `self` and producing a handle.
    fn start(self) -> TaskServiceHandle<I, O> {
        TaskServiceHandle::new(self)
    }
}

impl<I: Send + 'static, O: Send + 'static> TaskServiceHandle<I, O> {
    /// Create and start a service.
    pub fn new<S: TaskService<I, O> + Send + 'static>(mut service: S) -> Self {
        let (i_send, i_recv) = tokio_mpsc::unbounded_channel::<I>();
        let (o_send, o_recv) = tokio_mpsc::unbounded_channel::<O>();

        let inbox = Arc::new(TokioMutex::new(i_recv));
        let outbox = Arc::new(TokioMutex::new(o_send));

        let handle_inbox = inbox.clone();
        let handle_outbox = outbox.clone();
        let handle = task::spawn(async move {
            let mut i_send = i_send;
            let mut o_recv = o_recv;
            loop {
                service.run(o_recv, i_send).await;

                service.on_restart().await;

                let i_recv;
                let o_send;
                (i_send, i_recv) = tokio_mpsc::unbounded_channel::<I>();
                (o_send, o_recv) = tokio_mpsc::unbounded_channel::<O>();

                *handle_inbox.lock().await = i_recv;
                *handle_outbox.lock().await = o_send;
            }
        });

        TaskServiceHandle {
            inbox,
            outbox,
            handle,
        }
    }

    /// Receive a message from the service.
    pub async fn recv(&self) -> Option<I> {
        self.inbox.lock().await.recv().await
    }

    /// Send a message to the service.
    pub async fn send(&self, msg: O) {
        // TODO best error handling option is probably to wait for restart and
        // send the message again
        let _ = self.outbox.lock().await.send(msg);
    }
}

/// Handle for a service that runs concurrently, on a new thread.
pub struct ThreadServiceHandle<I, O> {
    inbox: Arc<StdMutex<std_mpsc::Receiver<I>>>,
    outbox: Arc<StdMutex<std_mpsc::Sender<O>>>,
    handle: thread::JoinHandle<Never>,
}

impl<I: Send + 'static, O: Send + 'static> ThreadServiceHandle<I, O> {
    /// Create and start a service.
    pub fn new<S: ThreadService<I, O> + Send + 'static>(mut service: S) -> Self {
        let (i_send, i_recv) = std_mpsc::channel::<I>();
        let (o_send, o_recv) = std_mpsc::channel::<O>();

        let inbox = Arc::new(StdMutex::new(i_recv));
        let outbox = Arc::new(StdMutex::new(o_send));

        let handle_inbox = inbox.clone();
        let handle_outbox = outbox.clone();
        let handle = thread::spawn(move || {
            let mut i_send = i_send;
            let mut o_recv = o_recv;
            loop {
                service.run(o_recv, i_send);

                service.on_restart();

                let i_recv;
                let o_send;
                (i_send, i_recv) = std_mpsc::channel::<I>();
                (o_send, o_recv) = std_mpsc::channel::<O>();

                *handle_inbox.lock().unwrap() = i_recv;
                *handle_outbox.lock().unwrap() = o_send;
            }
        });

        ThreadServiceHandle {
            inbox,
            outbox,
            handle,
        }
    }

    /// Receive a message from the service.
    pub async fn recv(&self) -> Option<I> {
        self.inbox.lock().unwrap().recv().ok()
    }

    /// Send a message to the service.
    pub async fn send(&self, msg: O) {
        // TODO same as `TaskServiceHandle::send`
        let _ = self.outbox.lock().unwrap().send(msg);
    }
}

/// A service that runs concurrently on a new thread.
/// 
/// `I` and `O` refer to the incoming/outgoing types for messages sent to the
/// handle. I.e., this type consumes `O`s and produces `I`s.
pub trait ThreadService<I: Send + 'static, O: Send + 'static> where Self: Sized + Send + 'static {
    /// Run the service. This is also the way that the service restarts.
    fn run(&mut self, inbox: std_mpsc::Receiver<O>, outbox: std_mpsc::Sender<I>);

    /// Optional restart callback, to help keep code cleaner. This will not get
    /// called the first time the service starts.
    fn on_restart(&mut self) {}

    /// Start the service, consuming `self` and producing a handle.
    fn start(self) -> ThreadServiceHandle<I, O> {
        ThreadServiceHandle::new(self)
    }
}