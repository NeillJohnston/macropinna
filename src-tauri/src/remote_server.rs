//! Interface for the remote control server.

use crate::config_listener::ConfigManager;
use serde::{Serialize, de::DeserializeOwned};
use shared::api::remote::*;

use tauri::State;
use uuid::Uuid;

async fn get_deserialized<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let res = reqwest::get(url).await?;
    let value = res.json().await?;
    let obj = serde_json::from_value(value)?;
    Ok(obj)
}

#[tauri::command]
pub async fn get_pending_info_list(config: State<'_, ConfigManager>) -> Result<Vec<AccessInfo>, ()> {
    let url = format!(
        "http://localhost:{}/api/current/pending",
        config.config.read().unwrap().remote_server.port_internal
    );

    get_deserialized(&url)
        .await
        .map_err(|err| {
            log::error!("{}", err);
        })
}

#[tauri::command]
pub async fn get_active_info_list(config: State<'_, ConfigManager>) -> Result<Vec<ActiveInfo>, ()> {
    let url = format!(
        "http://localhost:{}/api/current/active",
        config.config.read().unwrap().remote_server.port_internal
    );

    get_deserialized(&url)
        .await
        .map_err(|err| {
            log::error!("{}", err);
        })
}

#[tauri::command]
pub async fn update_pending(config: State<'_, ConfigManager>, uuid: Uuid, approve: bool) -> Result<(), ()> {
    let endpoint = match approve {
        true => "approve",
        false => "reject"
    };

    let url = format!(
        "http://localhost:{}/api/{}/{}",
        config.config.read().unwrap().remote_server.port_internal,
        endpoint,
        uuid
    );

    let client = reqwest::Client::new();
    client
        .post(url)
        .send()
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("{}", err);
        })
}

#[derive(Serialize)]
pub struct RemoteServerIp {
    name: String,
    ip: String,
}

#[tauri::command]
pub fn get_remote_server_ips(config: State<'_, ConfigManager>) -> Vec<RemoteServerIp> {
    use get_if_addrs::get_if_addrs;

    let port = config.config.read().unwrap().remote_server.port;

    get_if_addrs()
        .map_err(|err| {
            log::error!("Could not find network interfaces: {}", err);
        })
        .map(|net_ifs| net_ifs
            .into_iter()
            .filter(|net_if| !net_if.is_loopback() && net_if.ip().is_ipv4())
            .map(|net_if| RemoteServerIp {
                name: net_if.name.clone(),
                ip: format!("https://{}:{}", net_if.ip(), port)
            })
            .collect::<Vec<_>>()
        )
        .unwrap_or(vec![])
}