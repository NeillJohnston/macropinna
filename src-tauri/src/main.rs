// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_visualizer;
mod commands;
mod config;

fn main() -> anyhow::Result<()> {
    use config::ConfigManager;
    use audio_visualizer::AudioVisualizerManager;

    env_logger::init();

    // TODO test value here obvi
    let config_manager = ConfigManager::new("../config.json");
    
    let audio_visualizer_manager = {
        let config = config_manager.config.read().unwrap();
        AudioVisualizerManager::new(&config.audio_device.name)?
    };

    tauri::Builder::default()
        .manage(config_manager)
        .manage(audio_visualizer_manager)
        .invoke_handler(tauri::generate_handler![
            // commands::keystone_correct,
            commands::get_config,
            commands::set_config,
            commands::get_weather,
            commands::get_audio_spectrum,
            commands::get_player_metadata,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
