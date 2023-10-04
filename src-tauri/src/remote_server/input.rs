//! Handling for user input events.
//! 
//! This is mostly handled by the crate `tfc`. Since a `tfc::Context` isn't
//! thread-safe (it can't be, it needs access to file handles), it has to get
//! created in its own worker thread and have events wired to it.

use std::{
    sync::mpsc,
    thread::JoinHandle
};

use crate::util::numeric::try_f64_to_i32;

pub struct Context {
    send: mpsc::Sender<RemoteControlEvent>,
    _handle: JoinHandle<()>
}

#[derive(Debug, serde::Deserialize)]
pub enum RemoteControlEvent {
    DPad(DPadDirection),
    Text(String),
    Keyboard(Key),
    MouseMove {
        dx: f64,
        dy: f64,
    },
    MousePress(MouseButton)
}

#[derive(Debug, serde::Deserialize)]
pub enum DPadDirection {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Exit,
}

#[derive(Debug, serde::Deserialize)]
pub enum Key {
    Backspace,
    // Delete
}

#[derive(Debug, serde::Deserialize)]
pub enum MouseButton {
    LeftButton,
    RightButton
}

impl Context {
    pub fn new() -> Self {
        use std::thread::spawn;
        use tfc::traits::*;

        let (send, recv) = mpsc::channel();

        let handle = spawn(move || {
            let mut context = tfc::Context::new().unwrap();

            while let Ok(event) = recv.recv() {
                use RemoteControlEvent::*;
                use DPadDirection::*;
                use Key::*;
                use MouseButton::*;

                match event {
                    DPad(dir) => {
                        let key = match dir {
                            Up => tfc::Key::UpArrow,
                            Down => tfc::Key::DownArrow,
                            Left => tfc::Key::LeftArrow,
                            Right => tfc::Key::RightArrow,
                            Enter => tfc::Key::Space,
                            Exit => tfc::Key::Escape,
                        };

                        context.key_click(key).unwrap();
                    }
                    Text(text) => {
                        context.unicode_string(&text).unwrap();
                    }
                    Keyboard(key) => {
                        let key = match key {
                            Backspace => tfc::Key::DeleteOrBackspace
                        };

                        context.key_click(key).unwrap();
                    }
                    MouseMove { dx, dy } => {
                        let dx = try_f64_to_i32(dx);
                        let dy = try_f64_to_i32(dy);
                        
                        match (dx, dy) {
                            (Some(dx), Some(dy)) => {
                                context.mouse_move_rel(dx, dy).unwrap();
                            },
                            _ => {}
                        };
                    }
                    MousePress(button) => {
                        let button = match button {
                            LeftButton => tfc::MouseButton::Left,
                            RightButton => tfc::MouseButton::Right,
                        };

                        context.mouse_click(button).unwrap();
                    }
                }
            }
        });

        Context { send, _handle: handle }
    }

    pub fn play_event(&self, event: RemoteControlEvent) {
        // TODO handle error - maybe try to restart the input worker
        let _ = self.send.send(event);
    }
}
