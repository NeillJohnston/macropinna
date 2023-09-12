use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = {
        Arc::new(Mutex::new(Config::default()))
    };
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub weather: Option<Weather>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub provider: WeatherProvider,
    pub api_key: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap
}
