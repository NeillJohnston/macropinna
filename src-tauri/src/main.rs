// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use tauri::{AppHandle, Wry, Manager};

mod audio_visualizer;
mod config;
mod launcher;
mod suggest_launchers;
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
    use launcher::LauncherManager;
    use audio_visualizer::AudioVisualizerManager;
    use remote_server::RemoteServerManager;

    env_logger::init();

    let global_app_handle = GlobalAppHandle {
        handle: Arc::new(Mutex::new(None)),
        exists: Arc::new(AtomicBool::new(false))
    };

    // Initialize project directories, managers (which may have their own
    // initialization routines)

    init_directories();

    let config_manager = ConfigManager::new(global_app_handle.clone());

    let launcher_manager = LauncherManager::new();

    let audio_visualizer_manager = AudioVisualizerManager::new(&config_manager).unwrap();

    let remote_manager = RemoteServerManager::new(
        &config_manager,
        global_app_handle.clone()
    );
    
    let mut builder = tauri::Builder::default()
        .setup(move |app| {
            let mut handle = global_app_handle.handle.lock().unwrap();
            *handle = Some(app.handle());
            global_app_handle.exists.store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        })
        .manage(config_manager)
        .manage(launcher_manager)
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
            remote_server::ip::get_remote_server_ip,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
