use crate::config::ConfigManager;
use tauri::State;

#[tauri::command]
pub fn get_remote_server_ip(config: State<'_, ConfigManager>) -> Option<String> {
    use get_if_addrs::{get_if_addrs, IfAddr};

    // TODO here I'm trying to (very lazily) pick the first non-loopback device
    // with the "largest" subnet mask - I think this means I'll be picking the
    // most specific subnet which should hopefully resolve to a local-network
    // address. Really don't know much about this stuff though
    let addr = get_if_addrs()
        .map_err(|err| {
            log::error!("Could not find network interfaces: {}", err);
        })
        .ok()?
        .into_iter()
        .filter(|net_if| !net_if.is_loopback())
        .max_by_key(|net_if| match &net_if.addr {
            IfAddr::V4(v4) => v4.netmask.octets(),
            IfAddr::V6(_v6) => [0, 0, 0, 0],
        })?;

    log::info!("Using IP: {:?}", addr.ip());

    let port = config.config.read().unwrap().remote_server.port;
    let remote_address = format!("https://{}:{}", addr.ip(), port);

    Some(remote_address)
}