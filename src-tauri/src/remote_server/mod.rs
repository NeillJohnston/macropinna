//! Server for the remote control.
//! 
//! Serves the remote static site and handles client connections, all handled
//! through a single warp server.

mod input;

use crate::{
    GlobalAppHandle,
    config::ConfigManager, PROJECT_DIRS,
};

use futures::stream::SplitSink;
use serde::{Serialize, Deserialize};
use std::{
    collections::BTreeMap,
    convert::Infallible,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Instant, Duration},
};
use tauri::State;
use tokio::{sync::oneshot, time::timeout};
use uuid::Uuid;
use warp::{
    ws,
    reply::Reply, Filter,
};

const PENDING_TIMEOUT_S: u64 = 60_000;
pub struct RemoteServerManager {
    state: Arc<ServerState>,
}

struct ServerState {
    app_handle: GlobalAppHandle,
    port: u16,
    signer: Arc<String>,
    cert_path: Arc<PathBuf>,
    key_path: Arc<PathBuf>,
    input_ctx: input::Context,
    // State
    init_map: Mutex<BTreeMap<Uuid, AccessInit>>,
    pending_map: Mutex<BTreeMap<Uuid, AccessPending>>,
    active_map: Mutex<BTreeMap<Uuid, Active>>,
    // TODO store inactive devices (persistently) as well, this will require
    // require some sort of database to be added
}

/// The part of an access request that can be consumed by the frontend.
#[derive(Clone, Serialize)]
pub struct AccessInfo {
    uuid: Uuid,
    name: String,
    code: String
}

#[derive(Clone)]
struct AccessInit {
    info: AccessInfo
}

enum AccessApproval {
    Approve(AccessInfo),
    Reject
}

struct AccessPending {
    info: AccessInfo,
    timestamp: Instant,
    send: oneshot::Sender<AccessApproval>
}

/// The part of an active connection that can be consumed by the frontend.
#[derive(Clone, Serialize)]
pub struct ActiveInfo {
    uuid: Uuid,
    name: String,
    // TODO other stuff from claims
}

struct Active {
    info: ActiveInfo,
    send: SplitSink<ws::WebSocket, ws::Message>
}

impl ServerState {
    /// Add an initial connection
    fn add_init(&self, uuid: Uuid, name: String, code: String) {
        let mut init_map = self.init_map.lock().unwrap();

        init_map.insert(uuid.clone(), AccessInit {
            info: AccessInfo { uuid, name, code },
        });
    }

    /// Upgrade an initial connection to pending, returning its dedicated `Receiver`.
    fn upgrade_init(&self, uuid: Uuid) -> Option<oneshot::Receiver<AccessApproval>> {
        let init = { // Lock for init_map
            let mut init_map = self.init_map.lock().unwrap();
            init_map.remove(&uuid)?
        };

        let (send, recv) = oneshot::channel();
        { // Lock for pending_map
            let mut pending_map = self.pending_map.lock().unwrap();
            pending_map.insert(uuid, AccessPending {
                info: init.info,
                timestamp: Instant::now(),
                send
            });
        }

        self.app_handle.emit_all(
            RemoteServerEvent::channel(),
            RemoteServerEvent::RefreshPending
        );

        Some(recv)
    }

    /// Remove a pending connection
    fn remove_pending(&self, uuid: &Uuid) {
        { // Lock for pending_map
            let mut pending_map = self.pending_map.lock().unwrap();
            pending_map.remove(uuid);
        }
    
        self.app_handle.emit_all(
            RemoteServerEvent::channel(),
            RemoteServerEvent::RefreshPending
        );
    }

    /// Add an active connection
    fn add_active(&self, claims: ActiveDeviceClaims, send: SplitSink<ws::WebSocket, ws::Message>) {
        let info = ActiveInfo {
            uuid: claims.sub,
            name: claims.name
        };

        { // Lock for active_map
            let mut active_map = self.active_map.lock().unwrap();
            active_map.insert(claims.sub.clone(), Active {
                info: info.clone(),
                send
            });
        }

        self.app_handle.emit_all(
            RemoteServerEvent::channel(),
            RemoteServerEvent::Connected {
                uuid: info.uuid,
                name: info.name
            }
        );
    }

    /// Remote an active conncetion
    fn remove_active(&self, uuid: &Uuid) {
        { // Lock for active_map
            let mut active_map = self.active_map.lock().unwrap();
            active_map.remove(uuid);
        }

        self.app_handle.emit_all(
            RemoteServerEvent::channel(),
            RemoteServerEvent::RefreshActive
        );
    }
}

/// Request to register a device.
#[derive(Deserialize)]
struct RegisterRequest {
    device_name: String,
}

/// Response with a UUID and display code.
#[derive(Serialize)]
struct RegisterResponse {
    uuid: Uuid,
    code: String
}

/// Granted access via JWT.
#[derive(Serialize)]
struct AccessResponse {
    jwt: Option<String>
}

/// JWT claims object.
#[derive(Serialize, Deserialize)]
struct ActiveDeviceClaims {
    iss: String,
    sub: Uuid,
    name: String,
}

impl ActiveDeviceClaims {
    fn new(info: AccessInfo) -> Self {
        ActiveDeviceClaims {
            iss: "macropinna".to_string(),
            sub: info.uuid,
            name: info.name
        }
    }
}

impl RemoteServerManager {
    pub fn new(config: &ConfigManager, app_handle: GlobalAppHandle) -> Self {
        let config = config.config.read().unwrap();

        let cert_path = PROJECT_DIRS.config_dir().join("cert.pem");
        let key_path = PROJECT_DIRS.config_dir().join("key.pem");

        ensure_cert(&cert_path, &key_path).map_err(|err| {
            log::error!("Error while generating cert: {}", err);
        }).unwrap();

        let server = Arc::new(ServerState {
            app_handle,
            port: config.remote_server.port,
            // TODO unhardcode
            signer: Arc::new("secret".to_string()),
            cert_path: Arc::new(cert_path),
            key_path: Arc::new(key_path),
            input_ctx: input::Context::new(),
            init_map: Mutex::new(BTreeMap::new()),
            pending_map: Mutex::new(BTreeMap::new()),
            active_map: Mutex::new(BTreeMap::new()),
        });

        spawn_server(server.clone());

        RemoteServerManager {
            state: server
        }
    }
}

fn ensure_cert(cert_path: &PathBuf, key_path: &PathBuf) -> anyhow::Result<()> {
    use std::{
        fs::OpenOptions,
        io::Write
    };

    if !(cert_path.exists() && key_path.exists()) {
        log::info!("No cert found, generating a self-signed cert...");

        let cert = rcgen::generate_simple_self_signed(vec![
            "localhost".to_string()
        ])?;

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&cert_path)?
            .write_all(cert.serialize_pem()?.as_bytes())?;
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&key_path)?
            .write_all(cert.serialize_private_key_pem().as_bytes())?;
    }

    Ok(())
}

#[derive(Clone, Serialize)]
enum RemoteServerEvent {
    RefreshPending,
    RefreshActive,
    Connected {
        uuid: Uuid,
        name: String
    }
}

impl RemoteServerEvent {
    fn channel() -> &'static str {
        "remote_server"
    }
}

fn spawn_server(state: Arc<ServerState>) {
    tokio::task::spawn(async move {
        loop {
            let exists = &state.app_handle.exists;
            if exists.load(std::sync::atomic::Ordering::Relaxed) { break; }
        }

        let remote_static_path = state.app_handle.handle
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .path_resolver()
            .resolve_resource("remote-static")
            .unwrap();

        let index = warp::fs::dir(remote_static_path);

        let register = warp::path!("api" / "register")
            .and(warp::post())
            .and(warp::body::json::<RegisterRequest>())
            .and(with_state(state.clone()))
            .map(handle_register);

        let register_uuid = warp::path!("api" / "register" / Uuid)
            .and(warp::post())
            .and(with_state(state.clone()))
            .then(handle_register_uuid);

        let ws = warp::path!("api" / "ws" / String)
            .and(warp::ws())
            .and(with_state(state.clone()))
            .map(handle_ws);

        let routes = index
            .or(register)
            .or(register_uuid)
            .or(ws);

        warp::serve(routes)
            .tls()
            .cert_path(state.cert_path.as_path())
            .key_path(state.key_path.as_path())
            .run(([0, 0, 0, 0], state.port))
            .await;
    });
}

fn with_state(server: Arc<ServerState>) -> impl Filter<Extract = (Arc<ServerState>,), Error = Infallible> + Clone {
    warp::any().map(move || server.clone())
}

fn handle_register(req: RegisterRequest, state: Arc<ServerState>) -> impl Reply {
    let uuid = Uuid::new_v4();
    // Code is generated as the low 8 digits of a uuid, presumably random enough
    let code = Uuid::new_v4().as_u128() % 1_0000_0000;
    let code = format!("{:08}", code);

    state.add_init(uuid.clone(), req.device_name, code.clone());

    warp::reply::json(&RegisterResponse { uuid, code })
}

async fn handle_register_uuid(uuid: Uuid, state: Arc<ServerState>) -> impl Reply {
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use sha2::Sha256;

    // Custom cleanup struct to make sure we remove the `AccessPending` object
    // from `pending_map` even if we get dropped early
    struct Cleanup {
        uuid: Uuid,
        state: Arc<ServerState>
    }
    
    impl Drop for Cleanup {
        fn drop(&mut self) {
            self.state.remove_pending(&self.uuid);
        }
    }

    let recv = match state.upgrade_init(uuid.clone()) {
        Some(recv) => recv,
        None => {
            return warp::reply::with_status(
                "uuid not found",
                warp::http::StatusCode::UNAUTHORIZED
            ).into_response();
        }
    };

    let signer = state.signer.clone();
    let cleanup = Cleanup { uuid, state };
    
    // Wait for approval/rejection/timeout
    let res = match timeout(Duration::from_secs(PENDING_TIMEOUT_S), recv).await {
        Ok(Ok(AccessApproval::Approve(info))) => {
            log::info!("Approving access for {} ({})", info.name, info.uuid);

            let key: Hmac<Sha256> = Hmac::new_from_slice(signer.as_bytes()).unwrap();
            let claims = ActiveDeviceClaims::new(info);
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

fn handle_ws(jwt: String, ws: ws::Ws, state: Arc<ServerState>) -> impl Reply {
    use hmac::{Hmac, Mac};
    use jwt::VerifyWithKey;
    use sha2::Sha256;

    let signer = &state.signer;
    let key: Hmac<Sha256> = Hmac::new_from_slice(signer.as_bytes()).unwrap();

    let claims: ActiveDeviceClaims = match jwt.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(err) => {
            log::error!("Error during WebSocket verification: {}", err);
            return warp::reply::with_status(
                "unauthorized",
                warp::http::StatusCode::UNAUTHORIZED
            ).into_response();
        }
    };

    ws.on_upgrade(move |socket| {
        handle_ws_connection(socket, claims, state)
    }).into_response()
}

/// Handler for an active, authenticated WebSocket connection.
async fn handle_ws_connection(socket: ws::WebSocket, claims: ActiveDeviceClaims, state: Arc<ServerState>) {
    use futures::StreamExt;
    
    let (ws_send, mut ws_recv) = socket.split();

    let name = claims.name.clone();
    let uuid = claims.sub.clone();
    state.add_active(claims, ws_send);

    while let Some(msg) = ws_recv.next().await {
        match msg {
            Ok(msg) if msg.is_text() => {
                let msg = msg.to_str().unwrap();
                match serde_json::from_str::<input::RemoteControlEvent>(msg) {
                    Ok(event) => {
                        state.input_ctx.play_event(event);
                    }
                    Err(err) => {
                        log::error!("Error while deserializing WebSocket message \"{}\": {}", msg, err);
                    }
                }
            },
            Err(err) => {
                log::error!("Error while handling WebSocket connection: {}", err);
            },
            // Other messages (including binary messages) are ignored
            _ => {}
        }
    }

    log::info!("Connection dropped for {} ({})", name, uuid);
    state.remove_active(&uuid);
}

#[tauri::command]
pub fn get_pending_info_list(remote_server: State<'_, RemoteServerManager>) -> Vec<AccessInfo> {
    let mut list = { // Lock for server.pending_map
        let pending_map = remote_server.state.pending_map.lock().unwrap();
        pending_map.values().map(|val| (val.info.clone(), val.timestamp)).collect::<Vec<_>>()
    };

    list.sort_by_key(|(_, timestamp)| timestamp.clone());

    list.into_iter().map(|(info, _)| info).collect()
}

#[tauri::command]
pub fn get_active_info_list(remote_server: State<'_, RemoteServerManager>) -> Vec<ActiveInfo> {
    let list = { // Lock for server.active_map
        let active_map = remote_server.state.active_map.lock().unwrap();
        active_map.values().map(|val| val.info.clone()).collect::<Vec<_>>()
    };

    list
}

#[tauri::command]
pub fn update_pending(remote_server: State<'_, RemoteServerManager>, uuid: Uuid, approve: bool) {
    let pending = { // Lock for server.pending_map
        let mut pending_map = remote_server.state.pending_map.lock().unwrap();
        pending_map.remove(&uuid)
    };

    if pending.is_none() {
        log::error!("Tried to update pending status for nonexistent uuid {}", uuid);
        return;
    }
    let pending = pending.unwrap();
    
    let _res = pending.send.send(match approve {
        true => AccessApproval::Approve(pending.info),
        false => AccessApproval::Reject
    });
}