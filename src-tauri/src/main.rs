// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;

fn main() -> Result<(), ()> {
    use config::ConfigManager;

    env_logger::init();

    // TODO test value here obvi
    let config_manager = ConfigManager::new("../config.json");
    config_manager.save();

    tauri::Builder::default()
        .manage(config_manager)
        .invoke_handler(tauri::generate_handler![
            // commands::keystone_correct,
            commands::get_config,
            commands::set_config,
            commands::get_weather,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
