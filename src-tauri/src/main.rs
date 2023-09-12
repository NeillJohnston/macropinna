// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;

fn main() -> Result<(), ()> {
    use std::fs::read;
    use config::Config;

    env_logger::init();

    // TODO control config location by command line arg
    let config = match read("config.json") {
        Ok(bytes) => bytes,
        Err(err) => {
            log::error!("{}", err);
            return Err(());
        }
    };

    let config = match serde_json::from_slice::<Config>(&config) {
        Ok(config) => config,
        Err(err) => {
            log::error!("{}", err);
            return Err(());
        }
    };

    panic!("{:#?}", config);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::keystone_correct
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
