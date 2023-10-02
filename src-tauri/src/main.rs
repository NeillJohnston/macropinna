// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Wry, Manager};

mod audio_visualizer;
mod config;
mod media_player;
mod remote_server;
mod weather;
mod util;

/// Global handle for the Tauri app, primarily used to expose the events API to
/// to the remote server. Since Tauri only gives you access to events if you
/// have a handle to an app, and it only exposes this in a callback from the
/// `Builder`, we have to take an excessively convoluted route to allow other
/// threads access.
#[derive(Clone)]
pub struct GlobalAppHandle(Arc<Mutex<Option<AppHandle<Wry>>>>);

impl GlobalAppHandle {
    pub fn emit_all<P: Clone + serde::Serialize>(&self, channel: &'static str, payload: P) {
        let app_handle = self.0.lock().unwrap();
        let app_handle = app_handle.as_ref().unwrap();
        app_handle.emit_all(channel, payload).unwrap();
    } 
}

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs = {
        ProjectDirs::from(
            "",
            "Macropinna",
            "Macropinna"
        ).unwrap()
    };
}

/// Initialize base directories
fn init_directories() {
    use std::fs;

    if let Err(err) = fs::create_dir_all(PROJECT_DIRS.config_dir()) {
        log::error!("{}", err);
        panic!();
    }

    if let Err(err) = fs::create_dir_all(PROJECT_DIRS.data_dir()) {
        log::error!("{}", err);
        panic!();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use config::ConfigManager;
    use audio_visualizer::AudioVisualizerManager;
    use remote_server::RemoteServerManager;

    env_logger::init();

    let global_app_handle = GlobalAppHandle(Arc::new(Mutex::new(None)));

    // Initialize project directories, managers (which may have their own
    // initialization routines)

    init_directories();

    let config_manager = ConfigManager::new(global_app_handle.clone());

    let audio_visualizer_manager = AudioVisualizerManager::new(&config_manager).unwrap();

    let remote_manager = RemoteServerManager::new(
        &config_manager,
        global_app_handle.clone()
    );
    
    let mut builder = tauri::Builder::default()
        .setup(move |app| {
            let mut handle = global_app_handle.0.lock().unwrap();
            *handle = Some(app.handle());
            Ok(())
        })
        .manage(config_manager)
        .manage(audio_visualizer_manager)
        .manage(remote_manager);

    #[cfg(windows)]
    {
        let media_player_manager = media_player::MediaPlayerManager;
        builder = builder.manage(media_player_manager);
    }

    builder
        .invoke_handler(tauri::generate_handler![
            // commands::keystone_correct,
            config::get_config,
            config::set_config,
            weather::get_weather,
            audio_visualizer::get_audio_spectrum,
            media_player::get_player_metadata,
            remote_server::get_pending_info_list,
            remote_server::get_active_info_list,
            remote_server::update_pending,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
