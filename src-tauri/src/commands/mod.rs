use crate::config::*;

use serde_json::Value as JsonValue;
use tauri::State;

// TODO
// #[tauri::command]
// pub async fn keystone_correct(_angle: f64) {
//     use std::process::Command;

//     let (a, b, c, d, e, f, g, h, i) = (0, 0, 0, 0, 0, 0, 0, 0, 0);

//     let _ = Command::new("xrandr")
//         .arg("--transform")
//         .arg(format!("{a},{b},{c},{d},{e},{f},{g},{h},{i}"))
//         .output();
// }

#[tauri::command]
pub fn get_config(config: State<'_, ConfigManager>) -> Config {
    config.config.read().unwrap().clone()
}

#[tauri::command]
pub fn set_config(config: State<'_, ConfigManager>, new_config: Config) {
    *config.config.write().unwrap() = new_config;
    config.save();
}

#[tauri::command]
pub async fn get_weather(config: State<'_, ConfigManager>) -> Result<JsonValue, ()> {
    // TODO returning a JSON value is convenient, but causes unnecessary
    // conversions to/from string - 
    _get_weather(config)
        .await
        .map_err(|err| {
            log::error!("Error while fetching weather: {}", err);
            ()
        })
}

// Helper function that returns an `anyhow` error, which gets discarded by the
// actual command.
async fn _get_weather(config: State<'_, ConfigManager>) -> anyhow::Result<JsonValue> {
    // Scoping required so that the compiler knows when `config` gets dropped
    // https://github.com/rust-lang/rust/issues/63768
    let url;
    {
        let config = config.config.read().unwrap();
        url =
            if let Some(weather) = &config.weather {
                match weather.provider {
                    WeatherProvider::OpenWeatherMap => {
                        format!(
                            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
                            weather.lat,
                            weather.long,
                            weather.api_key
                        )
                    }
                }
            }
            else {
                anyhow::bail!("weather provider not configured");
            };
    }

    let res = reqwest::get(url)
        .await?
        .json()
        .await?;

    Ok(res)
}