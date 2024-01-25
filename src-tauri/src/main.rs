// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, atomic::AtomicBool};
use tauri::{AppHandle, Wry, Manager};

mod audio_visualizer;
mod config_listener;
mod launcher;
mod suggest_launchers;
mod media_player;
mod remote_server;
mod weather;

/// Global handle for the Tauri app. Since Tauri only gives you access to events
/// if you have a handle to an app, and it only exposes this in a callback from
/// the `Builder` or a command, we have to take an excessively convoluted route
/// to allow other threads access.
/// 
/// TODO this is only used to emit events from the config file watcher, which
/// could be replaced with a poll-type watcher to save some hassle
#[derive(Clone)]
pub struct GlobalAppHandle {
    handle: Arc<Mutex<Option<AppHandle<Wry>>>>,
    exists: Arc<AtomicBool>
}

impl GlobalAppHandle {
    pub fn emit_all<P: Clone + serde::Serialize>(&self, channel: &'static str, payload: P) {
        let app_handle = self.handle.lock().unwrap();
        let app_handle = app_handle.as_ref().unwrap();
        app_handle.emit_all(channel, payload).unwrap();
    } 
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use config_listener::ConfigManager;
    use launcher::LauncherManager;
    use audio_visualizer::AudioVisualizerManager;
    use shared::util::project_dirs;

    env_logger::init();

    let global_app_handle = GlobalAppHandle {
        handle: Arc::new(Mutex::new(None)),
        exists: Arc::new(AtomicBool::new(false))
    };

    // Initialize project directories, managers (which may have their own
    // initialization routines)

    project_dirs::ensure();

    let config_manager = ConfigManager::new(global_app_handle.clone());

    let launcher_manager = LauncherManager::new();

    let audio_visualizer_manager = AudioVisualizerManager::new(&config_manager).unwrap();
    
    let mut builder = tauri::Builder::default()
        .setup(move |app| {
            let mut handle = global_app_handle.handle.lock().unwrap();
            *handle = Some(app.handle());
            global_app_handle.exists.store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        })
        .manage(config_manager)
        .manage(launcher_manager)
        .manage(audio_visualizer_manager);

    #[cfg(windows)]
    {
        let media_player_manager = media_player::MediaPlayerManager;
        builder = builder.manage(media_player_manager);
    }

    builder
        .invoke_handler(tauri::generate_handler![
            // commands::keystone_correct,
            config_listener::get_config,
            config_listener::set_config,
            launcher::launch,
            suggest_launchers::suggest_launchers,
            weather::get_weather,
            audio_visualizer::get_audio_spectrum,
            audio_visualizer::pause_audio_visualizer,
            audio_visualizer::unpause_audio_visualizer,
            media_player::get_player_metadata,
            remote_server::get_pending_info_list,
            remote_server::get_active_info_list,
            remote_server::update_pending,
            remote_server::get_remote_server_ips,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
