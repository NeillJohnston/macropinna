use crate::config_listener::ConfigManager;
use shared::config::*;

use serde::Serialize;
use serde_json::Value as JsonValue;
use tauri::State;

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