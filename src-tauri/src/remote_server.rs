//! Interface for the remote control server.

use crate::config_listener::ConfigManager;
use shared::api::remote::*;

use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_pending_info_list(app_handle: tauri::AppHandle, config: State<'_, ConfigManager>) -> Vec<AccessInfo> {
    todo!()
}

#[tauri::command]
pub fn get_active_info_list(app_handle: tauri::AppHandle, config: State<'_, ConfigManager>) -> Vec<ActiveInfo> {
    todo!()
}

#[tauri::command]
pub fn update_pending(app_handle: tauri::AppHandle, config: State<'_, ConfigManager>, uuid: Uuid, approve: bool) {
    todo!()
}

#[tauri::command]
pub fn get_remote_server_ips(config: State<'_, ConfigManager>) -> Vec<String> {
    use get_if_addrs::get_if_addrs;

    let port = config.config.read().unwrap().remote_server.port;

    get_if_addrs()
        .map_err(|err| {
            log::error!("Could not find network interfaces: {}", err);
        })
        .map(|net_ifs| net_ifs
            .into_iter()
            .filter(|net_if| !net_if.is_loopback() && net_if.ip().is_ipv4())
            .map(|net_if| {
                format!("https://{}:{}", net_if.ip(), port)
            })
            .collect::<Vec<_>>()
        )
        .unwrap_or(vec![])
}