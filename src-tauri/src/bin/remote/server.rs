//! Server for the remote control.
//! 
//! The functionality is actually split into two warp servers:
//! - An external (broadcast) HTTPS endpoint
//! - An internal (localhost) HTTP endpoint
//! 
//! The external endpoint is what devices connect to, and the internal endpoint
//! is what the Macropinna UI calls.

use crate::input;
use shared::{
    config::Config,
    util::project_dirs::PROJECT_DIRS
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
use tokio::{sync::oneshot, time::timeout};
use uuid::Uuid;
use warp::{
    ws,
    reply::Reply, Filter,
};

const PENDING_TIMEOUT_S: u64 = 60_000;

struct ServerState {
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
    agent: Agent,
    code: String
}

/// Types of user agents (guessed based on User-Agent header - not a security thing)
#[derive(Clone, Serialize, Deserialize)]
pub enum Agent {
    Android,
    IPhone,
    Desktop,
    Unknown
}

impl From<String> for Agent {
    fn from(value: String) -> Self {
        // Why not add a library to parse user agent strings? It just doesn't
        // feel necessary, this is intended to be a fun little extra bit of info
        // to display, and I don't care too much about accuracy
        if value.contains("Android") {
            Agent::Android
        }
        else if value.contains("iPhone") {
            Agent::IPhone
        }
        else if value.contains("Windows") || value.contains("Macintosh") || value.contains("X11") {
            Agent::Desktop
        }
        else {
            Agent::Unknown
        }
    }
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
    agent: Agent
}

struct Active {
    info: ActiveInfo,
    _send: SplitSink<ws::WebSocket, ws::Message>
}

impl ServerState {
    /// Add an initial connection
    fn add_init(&self, uuid: Uuid, name: String, agent: Agent, code: String) {
        let mut init_map = self.init_map.lock().unwrap();

        init_map.insert(uuid.clone(), AccessInit {
            info: AccessInfo { uuid, name, agent, code },
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

        Some(recv)
    }

    /// Remove a pending connection
    fn remove_pending(&self, uuid: &Uuid) {
        { // Lock for pending_map
            let mut pending_map = self.pending_map.lock().unwrap();
            pending_map.remove(uuid);
        }
    }

    /// Add an active connection
    fn add_active(&self, claims: ActiveDeviceClaims, send: SplitSink<ws::WebSocket, ws::Message>) {
        let info = ActiveInfo {
            uuid: claims.sub,
            name: claims.name,
            agent: claims.agent
        };

        { // Lock for active_map
            let mut active_map = self.active_map.lock().unwrap();
            active_map.insert(claims.sub.clone(), Active {
                info: info.clone(),
                _send: send
            });
        }
    }

    /// Remove an active conncetion
    fn remove_active(&self, uuid: &Uuid) {
        { // Lock for active_map
            let mut active_map = self.active_map.lock().unwrap();
            active_map.remove(uuid);
        }
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
    agent: Agent,
}

impl ActiveDeviceClaims {
    fn new(info: AccessInfo) -> Self {
        ActiveDeviceClaims {
            iss: "macropinna".to_string(),
            sub: info.uuid,
            name: info.name,
            agent: info.agent
        }
    }
}

pub async fn run(config: Config) {
    let cert_path = PROJECT_DIRS.config_dir().join("cert.pem");
    let key_path = PROJECT_DIRS.config_dir().join("key.pem");

    ensure_cert(&cert_path, &key_path).map_err(|err| {
        log::error!("Error while generating cert: {}", err);
    }).unwrap();

    let state = Arc::new(ServerState {
        // TODO unhardcode
        signer: Arc::new("secret".to_string()),
        cert_path: Arc::new(cert_path),
        key_path: Arc::new(key_path),
        input_ctx: input::Context::new(),
        init_map: Mutex::new(BTreeMap::new()),
        pending_map: Mutex::new(BTreeMap::new()),
        active_map: Mutex::new(BTreeMap::new()),
    });

    let remote_static_path = "./remote-static";

    // <external>/...: serves all static files
    let index = warp::fs::dir(remote_static_path);

    // <external>/api/register: register a device as a remote
    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::header("User-Agent"))
        .and(warp::body::json::<RegisterRequest>())
        .and(with_state(state.clone()))
        .map(handle_register);

    // <external>/api/register/[uuid]: wait for registration approval/rejection
    let register_uuid = warp::path!("api" / "register" / Uuid)
        .and(warp::post())
        .and(with_state(state.clone()))
        .then(handle_register_uuid);

    // <external>/api/ws/[jwt]: authenticates and creates a WebSocket connection
    let ws = warp::path!("api" / "ws" / String)
        .and(warp::ws())
        .and(with_state(state.clone()))
        .map(handle_ws);

    let routes = index
        .or(register)
        .or(register_uuid)
        .or(ws);

    let external = warp::serve(routes)
        .tls()
        .cert_path(state.cert_path.as_path())
        .key_path(state.key_path.as_path())
        .run(([0, 0, 0, 0], config.remote_server.port));

    // <internal>/api/approve/[uuid]: approve a device
    let approve = warp::path!("api" / "update" / Uuid)
        .and(warp::post())
        .and(with_state(state.clone()))
        .map(handle_approve);

    // <internal>/api/reject/[uuid]: reject a device
    let reject = warp::path!("api" / "update" / Uuid)
        .and(warp::post())
        .and(with_state(state.clone()))
        .map(handle_reject);

    // <internal>/api/current/pending: get the current pending connections
    let current_pending = warp::path!("api" / "current" / "pending")
        .and(with_state(state.clone()))
        .map(handle_current_pending);

    // <internal>/api/current/active: get the current active connections
    let current_active = warp::path!("api" / "current" / "active")
        .and(with_state(state.clone()))
        .map(handle_current_active);

    let routes = approve
        .or(reject)
        .or(current_pending)
        .or(current_active);

    let internal = warp::serve(routes)
        .run(([127, 0, 0, 1], config.remote_server.port_internal));

    futures::join!(external, internal);
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

fn with_state(server: Arc<ServerState>) -> impl Filter<Extract = (Arc<ServerState>,), Error = Infallible> + Clone {
    warp::any().map(move || server.clone())
}

fn handle_register(user_agent: String, req: RegisterRequest, state: Arc<ServerState>) -> impl Reply {
    let uuid = Uuid::new_v4();
    // Code is generated as the low 8 digits of a uuid, presumably random enough
    let code = Uuid::new_v4().as_u128() % 1_0000_0000;
    let code = format!("{:08}", code);

    state.add_init(uuid.clone(), req.device_name, user_agent.into(), code.clone());

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

fn handle_approve(uuid: Uuid, state: Arc<ServerState>) -> impl Reply {
    let pending = { // Lock for state.pending_map
        let mut pending_map = state.pending_map.lock().unwrap();
        pending_map.remove(&uuid)
    };

    if pending.is_none() {
        log::error!("Tried to update pending status for nonexistent uuid {}", uuid);
        return warp::reply::json(&false);
    }
    let pending = pending.unwrap();
    
    let res = pending.send.send(AccessApproval::Approve(pending.info));

    warp::reply::json(&res.is_ok())
}

fn handle_reject(uuid: Uuid, state: Arc<ServerState>) -> impl Reply {
    let pending = { // Lock for state.pending_map
        let mut pending_map = state.pending_map.lock().unwrap();
        pending_map.remove(&uuid)
    };

    if pending.is_none() {
        log::error!("Tried to update pending status for nonexistent uuid {}", uuid);
        return warp::reply::json(&false);
    }
    let pending = pending.unwrap();
    
    let res = pending.send.send(AccessApproval::Reject);

    warp::reply::json(&res.is_ok())
}

fn handle_current_pending(state: Arc<ServerState>) -> impl Reply {
    let mut list = { // Lock for state.pending_map
        let pending_map = state.pending_map.lock().unwrap();
        pending_map.values().map(|val| (val.info.clone(), val.timestamp)).collect::<Vec<_>>()
    };

    list.sort_by_key(|(_, timestamp)| timestamp.clone());

    let list = list.into_iter().map(|(info, _)| info).collect::<Vec<_>>();

    warp::reply::json(&list)
}

fn handle_current_active(state: Arc<ServerState>) -> impl Reply {
    let list = { // Lock for state.active_map
        let active_map = state.active_map.lock().unwrap();
        active_map.values().map(|val| val.info.clone()).collect::<Vec<_>>()
    };

    warp::reply::json(&list)
}