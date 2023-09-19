use crate::{config::*, audio_visualizer::*};

use serde::Serialize;
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

#[derive(Serialize)]
pub struct WeatherResponse {
    pub current: JsonValue,
    pub forecast: JsonValue
}

#[tauri::command]
pub async fn get_weather(config: State<'_, ConfigManager>) -> Result<WeatherResponse, ()> {
    _get_weather(config)
        .await
        .map_err(|err| {
            log::error!("Error while fetching weather: {}", err);
            ()
        })
}

// Helper function that returns an `anyhow` error, which gets discarded by the
// actual command.
async fn _get_weather(config: State<'_, ConfigManager>) -> anyhow::Result<WeatherResponse> {
    use futures::try_join;

    // Scoping required so that the compiler knows when `config` gets dropped
    // https://github.com/rust-lang/rust/issues/63768
    let weather = {
        let config = config.config.read().unwrap();
        
        if let Some(weather) = &config.weather {
            weather.clone()
        }
        else {
            anyhow::bail!("weather provider not configured");
        }
    };

    match weather.provider {
        WeatherProvider::OpenWeatherMap => {
            let current_url = format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
                weather.lat,
                weather.long,
                weather.api_key
            );
            let forecast_url = format!(
                "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
                weather.lat,
                weather.long,
                weather.api_key
            );

            let (current, forecast) = try_join!(
                reqwest::get(current_url),
                reqwest::get(forecast_url)
            )?;

            let current = current.json().await?;
            let forecast = forecast.json().await?;

            Ok(WeatherResponse { current, forecast })
        }
    }
}

#[derive(Serialize)]
pub struct AudioSpectrumResponse {
    pub data: Vec<f32>
}

#[tauri::command]
pub fn get_audio_spectrum(audio_visualizer: State<'_, AudioVisualizerManager>) -> AudioSpectrumResponse {
    let data = audio_visualizer.data.lock().unwrap();
    AudioSpectrumResponse { data: data.data.clone() }
}

#[derive(Serialize)]
pub struct PlayerMetadataResponse {
    album_name: Option<String>,
    disc_number: Option<i32>,
    track_title: Option<String>,
    track_number: Option<i32>,
    track_artists: Option<Vec<String>>,
    track_length_us: Option<u64>,
    art_url: Option<String>,
}

#[tauri::command]
pub fn get_player_metadata() -> Option<PlayerMetadataResponse> {
    use mpris::PlayerFinder;

    let player = PlayerFinder::new()
        .ok()?
        .find_active()
        .ok()?;

    let metadata = player
        .get_metadata()
        .ok()?;

    Some(PlayerMetadataResponse {
        album_name: metadata.album_name().map(|s| s.into()),
        disc_number: metadata.disc_number(),
        track_title: metadata.title().map(|s| s.into()),
        track_number: metadata.track_number(),
        track_artists: metadata.artists().map(|v| v.into_iter().map(|s| s.into()).collect()),
        track_length_us: metadata.length_in_microseconds(),
        art_url: metadata.art_url().map(|s| s.into()),
    })
}