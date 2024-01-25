use serde::{Serialize, Deserialize};
use std::path::Path;

/// Trait to provide a versioning chain for config files. Ensures that a config
/// file written for any config version can be safely used and upgraded to the
/// current version, while malformed config files will throw an error.
/// 
/// If deserialization fails for every version, then only the serde errors for
/// the first deserialization attempt are passed.
trait VersionedDeserialize<'a>: Deserialize<'a> {
    type Prev: VersionedDeserialize<'a>;

    fn versioned_deserialize(v: &'a [u8]) -> serde_json::Result<Self> {
        match serde_json::from_slice::<Self>(v) {
            Err(err) => {
                Self::Prev::versioned_deserialize(v)
                    .map(Self::upgrade)
                    .map_err(|_| err)
            }
            ok => ok
        }
    }

    fn upgrade(prev: Self::Prev) -> Self;
}

// Base case, the first config version will use `()` as the "previous version"
impl<'a> VersionedDeserialize<'a> for () {
    type Prev = ();

    // Always throws an error (which always gets ignored)
    fn versioned_deserialize(_v: &'a [u8]) -> serde_json::Result<Self> {
        // Was too lazy to figure out how to construct a serde error myself
        serde_json::from_str("{")
    }

    fn upgrade(_prev: Self::Prev) -> Self {
        ()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub name: String,
    pub theme: Theme,
    pub home: Home,
    pub launchers: Vec<Launcher>,
    pub shell: Option<String>,
    pub weather: Option<Weather>,
    pub audio_device: Option<AudioDevice>,
    pub remote_server: RemoteServer,
    pub needs_setup: Option<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    color: String,
    style: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Home {
    pub screens: Vec<Screen>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Screen {
    pub widgets: Vec<Widget>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Widget {
    pub name: String,
    pub coords: Coords,
    pub props: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coords {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Launcher {
    pub name: String,
    pub command: String,
    pub finder: String,
    pub finder_is_regex: Option<bool>,
    pub image_path: Option<String>,
    pub css_background: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weather {
    pub provider: WeatherProvider,
    pub api_key: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteServer {
    pub port: u16,
    pub port_internal: u16
}

impl Default for RemoteServer {
    fn default() -> Self {
        RemoteServer {
            port: 5174,
            port_internal: 51740
        }
    }
}

impl<'a> VersionedDeserialize<'a> for ConfigV1 {
    type Prev = ();

    fn upgrade(_prev: Self::Prev) -> Self {
        ConfigV1::default()
    }
}

pub type Config = ConfigV1;

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("default.json")).unwrap()
    }
}

pub fn save_to_path<P: AsRef<Path>>(path: P, config: &Config) {
    use serde_json::ser::Serializer;
    use std::fs::OpenOptions;

    let oo = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path);
    let file = match oo {
        Ok(file) => file,
        Err(err) => {
            log::error!("{}", err);
            return;
        }
    };

    let mut serializer = Serializer::pretty(file);
    if let Err(err) = config.serialize(&mut serializer) {
        log::error!("{}", err);
    }
}

pub fn load_from_path<P: AsRef<Path> + Clone>(path: P) -> Option<Config> {
    use std::fs::read;

    log::info!("Reading config from {}", path.as_ref().display());

    let config = match read(path.clone()) {
        Ok(bytes) => bytes,
        Err(err) => {
            log::error!("Error while reading config file: {}", err);
            "{version:0}".into()
        }
    };

    Config::versioned_deserialize(&config)
        .map_err(|err| {
            log::error!("Error while reading config file: {}", err);
        })
        .ok()
}