//! Server for the remote control.
//! 
//! Serves the remote static site and handles client connections, all handled
//! through a single warp server.

use crate::{
    GlobalAppHandle,
    config::ConfigManager,
};

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    convert::Infallible,
    time::{Instant, Duration}
};
use tauri::State;
use tokio::{sync::oneshot, time::timeout};
use uuid::Uuid;
use warp::{
    ws::Ws,
    reply::Reply, Filter
};

const PENDING_TIMEOUT_S: u64 = 60;

pub struct RemoteServerManager {
    server: Arc<Server>,
}

struct Server {
    app_handle: GlobalAppHandle,
    port: u16,
    key: Arc<String>,
    init_map: Mutex<BTreeMap<Uuid, AccessInit>>,
    pending_map: Mutex<BTreeMap<Uuid, AccessPending>>,
}

#[derive(Clone, serde::Serialize)]
pub struct DeviceInfo {
    uuid: Uuid,
    name: String,
    code: String
}

#[derive(Clone)]
struct AccessInit {
    timestamp: Instant,
    device: DeviceInfo
}

enum AccessApproval {
    Approve,
    Reject
}

struct AccessPending {
    init: AccessInit,
    send: oneshot::Sender<AccessApproval>
}

/// Request to register a device.
#[derive(serde::Deserialize)]
struct RegisterRequest {
    device_name: String,
}

/// Response with a UUID and display code.
#[derive(serde::Serialize)]
struct RegisterResponse {
    uuid: Uuid,
    code: String
}

/// Granted access via JWT.
#[derive(serde::Serialize)]
struct AccessResponse {
    jwt: Option<String>
}

struct ActiveDevice {
    uuid: Uuid,
}

impl ActiveDevice {
    fn claims(&self) -> BTreeMap<&'static str, String> {
        BTreeMap::from([
            ("iss", "macropinna".to_string()),
            ("sub", self.uuid.to_string())
        ])
    }
}

impl RemoteServerManager {
    pub fn new(config: &ConfigManager, app_handle: GlobalAppHandle) -> Self {
        let config = config.config.read().unwrap();
        let server = Arc::new(Server {
            app_handle,
            port: config.remote_server.port,
            // TODO unhardcode
            key: Arc::new("secret".to_string()),
            init_map: Mutex::new(BTreeMap::new()),
            pending_map: Mutex::new(BTreeMap::new()),
        });

        spawn_server(server.clone());

        RemoteServerManager {
            server
        }
    }
}

#[derive(Clone, serde::Serialize)]
enum RemoteServerEvent {
    RefreshPending,
    RefreshActive,
    Connected {
        device: DeviceInfo
    }
}

impl RemoteServerEvent {
    fn channel() -> &'static str {
        "remote_server"
    }
}

fn spawn_server(server: Arc<Server>) {
    tokio::task::spawn(async move {
        let index = warp::fs::dir("./remote-static");
    
        let register = warp::path!("api" / "register")
            .and(warp::post())
            .and(warp::body::json::<RegisterRequest>())
            .and(with_server(server.clone()))
            .map(handle_register);

        // Per https://security.stackexchange.com/questions/7705/does-ssl-tls-https-hide-the-urls-being-accessed:
        // HTTPS does hide URLs, so using the uuid as a secret *should* be safe.
        // However, honestly, it feels wrong, so I might change this later -
        // maybe another JWT?
        let register_uuid = warp::path!("api" / "register" / Uuid)
            .and(warp::post())
            .and(with_server(server.clone()))
            .then(handle_register_uuid);

        let ws = warp::path!("api" / "ws")
            .and(warp::ws())
            .and(warp::header::<String>("Authorization"))
            .and(with_server(server.clone()))
            .map(handle_ws);

        let routes = index
            .or(register)
            .or(register_uuid)
            .or(ws);

        warp::serve(routes).run(([127, 0, 0, 1], server.port)).await;
    });
}

fn with_server(server: Arc<Server>) -> impl Filter<Extract = (Arc<Server>,), Error = Infallible> + Clone {
    warp::any().map(move || server.clone())
}

fn handle_register(req: RegisterRequest, server: Arc<Server>) -> impl Reply {
    let uuid = Uuid::new_v4();
    // Code is generated as the low 8 digits of a uuid, presumably random enough
    // (Also split into 2 groups of digits for readability)
    let code_uuid = Uuid::new_v4().as_u128();
    let lower = code_uuid % 10000;
    let upper = (code_uuid / 10000) % 10000;
    let code = format!("{:04}-{:04}", upper, lower);

    { // Lock for server.pending
        let mut init_map = server.init_map.lock().unwrap();
        // TODO is there any sense in checking for collisions here
        init_map.insert(uuid.clone(), AccessInit {
            timestamp: Instant::now(),
            device: DeviceInfo {
                // (Note: redundant UUID storage)
                uuid,
                name: req.device_name,
                code: code.clone(),
            },
        });
    }

    warp::reply::json(&RegisterResponse { uuid, code })
}

async fn handle_register_uuid(uuid: Uuid, server: Arc<Server>) -> impl Reply {
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use sha2::Sha256;

    // Custom cleanup struct to make sure we remove the `AccessPending` object
    // from `pending_map` even if we get dropped early
    // TODO not sure if putting the struct/impl in here is cool or unhinged
    struct Cleanup {
        uuid: Uuid,
        server: Arc<Server>
    }
    
    impl Drop for Cleanup {
        fn drop(&mut self) {
            { // Lock for server.pending_map
                let mut pending_map = self.server.pending_map.lock().unwrap();
                pending_map.remove(&self.uuid);
            }
        
            self.server.app_handle.emit_all(
                RemoteServerEvent::channel(),
                RemoteServerEvent::RefreshPending
            );
        }
    }
    
    let init = { // Lock for server.init_map
        let mut init_map = server.init_map.lock().unwrap();

        match init_map.remove(&uuid) {
            Some(init) => init,
            None => {
                return warp::reply::with_status(
                    "uuid not found",
                    warp::http::StatusCode::UNAUTHORIZED
                ).into_response();
            }
        }
    };
    
    let (send, recv) = oneshot::channel();
    { // Lock for server.pending_map
        let mut pending_map = server.pending_map.lock().unwrap();
        pending_map.insert(uuid, AccessPending {
            init,
            send
        });
    }

    server.app_handle.emit_all(
        RemoteServerEvent::channel(),
        RemoteServerEvent::RefreshPending
    );

    let key = server.key.clone();
    let cleanup = Cleanup { uuid: uuid.clone(), server };
    
    // Wait for approval/rejection/timeout
    let res = match timeout(Duration::from_secs(PENDING_TIMEOUT_S), recv).await {
        Ok(Ok(AccessApproval::Approve)) => {
            log::info!("Approving access for {}", uuid);

            let device = ActiveDevice { uuid };
            
            let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();
            let claims = device.claims();

            let jwt = claims.sign_with_key(&key).unwrap();

            warp::reply::json(&AccessResponse {
                jwt: Some(jwt)
            }).into_response()
        }
        Ok(Ok(AccessApproval::Reject)) => {
            warp::reply::json(&AccessResponse {
                jwt: None
            }).into_response()
        }
        Ok(Err(_)) => {
            warp::reply::with_status(
                "internal error",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR
            ).into_response()
        }
        Err(_) => {
            log::info!(
                "Rejecting access for {} by default after {}s",
                uuid,
                PENDING_TIMEOUT_S
            );

            warp::reply::json(&AccessResponse {
                jwt: None
            }).into_response()
        }
    };

    drop(cleanup);
    res
}

fn handle_ws(_ws: Ws, _authorization: String, _server: Arc<Server>) -> impl Reply {
    "todo"
}

#[tauri::command]
pub fn get_pending_list(remote_server: State<'_, RemoteServerManager>) -> Vec<DeviceInfo> {
    let mut list = { // Lock for server.pending_map
        let pending = remote_server.server.pending_map.lock().unwrap();
        pending.values().map(|val| val.init.clone()).collect::<Vec<_>>()
    };

    list.sort_by_key(|init| init.timestamp);

    list.into_iter().map(|init| init.device).collect()
}

#[tauri::command]
pub fn update_pending(remote_server: State<'_, RemoteServerManager>, uuid: Uuid, approve: bool) {
    let pending = { // Lock for server.pending_map
        let mut pending_map = remote_server.server.pending_map.lock().unwrap();
        pending_map.remove(&uuid)
    };

    if pending.is_none() {
        log::error!("Tried to update pending status for nonexistent uuid {}", uuid);
        return;
    }
    let pending = pending.unwrap();
    
    let _res = pending.send.send(match approve {
        true => AccessApproval::Approve,
        false => AccessApproval::Reject
    });
}