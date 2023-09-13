use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
// use tokio::sync::RwLock;

pub struct ConfigManager {
    path: PathBuf,
    pub config: RwLock<Config>
}

impl ConfigManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let config = Self::_load(&path);

        ConfigManager {
            path: path.as_ref().into(),
            config: RwLock::new(config)
        }
    }

    pub fn save(&self) {
        use serde_json::ser::Serializer;
        use std::fs::OpenOptions;

        let oo = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path);
        let file = match oo {
            Ok(file) => file,
            Err(err) => {
                log::error!("{}", err);
                return;
            }
        };

        let mut serializer = Serializer::pretty(file);
        if let Err(err) = self.config.read().unwrap().serialize(&mut serializer) {
            log::error!("{}", err);
        }
    }

    fn _load<P: AsRef<Path>>(path: P) -> Config {
        use std::fs::read;

        let config = match read(path) {
            Ok(bytes) => bytes,
            Err(err) => {
                log::error!("Error while reading config file: {}", err);
                "{version:0}".into()
            }
        };

        match Config::versioned_deserialize(&config) {
            Ok(config) => config,
            Err(err) => {
                log::error!("Fatal error while reading config file: {}", err);
                panic!();
            }
        }
    }
    
    pub fn load(&self) {
        let config = Self::_load(&self.path);
        *self.config.write().unwrap() = config;
    }
}

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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub name: String,
    pub weather: Option<Weather>
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

impl<'a> VersionedDeserialize<'a> for ConfigV1 {
    type Prev = ();

    fn upgrade(_prev: Self::Prev) -> Self {
        ConfigV1::default()
    }
}

pub type Config = ConfigV1;