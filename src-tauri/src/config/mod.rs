use notify::RecommendedWatcher;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tauri::State;

use crate::{PROJECT_DIRS, GlobalAppHandle};

pub struct ConfigManager {
    pub config: Arc<RwLock<Config>>,
    path: PathBuf,
    _watcher: notify::RecommendedWatcher
}

#[derive(Clone, Serialize)]
enum ConfigEvent {
    Set {
        config: Config
    }
}

impl ConfigEvent {
    fn channel() -> &'static str {
        "config"
    }
}

impl ConfigManager {
    pub fn new(app_handle: GlobalAppHandle) -> Self {

        // TODO definitely redundant if this is only ever going to be hardcoded.
        // Might be nice to let users change this based on environment vars, but
        // I'm not sure if that's a common usecase
        let base_path = PROJECT_DIRS.config_dir().into();
        let path = PROJECT_DIRS.config_dir().join("config.json");
        log::info!("Config path: {}", path.display());

        let config =
            if !path.exists() {
                let config = Config::default();

                Self::save_to_path(&path, &config);

                config
            }
            else {
                Self::load_from_path(&path).unwrap()
            };
        let config = Arc::new(RwLock::new(config));

        let watcher = Self::make_watcher(config.clone(), app_handle, base_path, path.clone());

        let config_manager = ConfigManager {
            config,
            path,
            _watcher: watcher,
        };

        config_manager
    }

    fn make_watcher(
        config_rw: Arc<RwLock<Config>>,
        app_handle: GlobalAppHandle,
        base_path: PathBuf,
        path: PathBuf
    ) -> RecommendedWatcher {
        use notify::{Event, RecursiveMode, Result, Watcher};

        let mut watcher = notify::recommended_watcher(
            move |res: Result<Event>| {
                if let Ok(event) = res {
                    // TODO only trigger on changes to actual file (path)
                    if event.kind.is_modify() {
                        if let Some(config) = ConfigManager::load_from_path(&path) {
                            log::info!("Detected change in config");
                            *config_rw.write().unwrap() = config.clone();

                            app_handle.emit_all(ConfigEvent::channel(), ConfigEvent::Set { config });
                        }
                    }
                }
            }
        ).unwrap();

        // When watching a single file, the inotify watcher deletes itself when
        // the original file is deleted - some editors handle file modifications
        // by deleting the original and creating a new file with the same name.
        // Not sure what the best way to handle that is, but watching something
        // that doesn't get deleted (like the directory) is better
        watcher.watch(&base_path, RecursiveMode::Recursive).unwrap();

        watcher
    }

    pub fn save(&self) {
        Self::save_to_path(&self.path, &self.config.read().unwrap());
    }

    fn save_to_path<P: AsRef<Path>>(path: P, config: &Config) {
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

    fn load_from_path<P: AsRef<Path> + Clone>(path: P) -> Option<Config> {
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

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub name: String,
    pub home: Home,
    pub launchers: Vec<Launcher>,
    pub shell: Option<String>,
    pub weather: Option<Weather>,
    pub audio_device: Option<AudioDevice>,
    pub remote_server: RemoteServer,
    pub needs_setup: Option<bool>
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
    pub port: u16
}

impl Default for RemoteServer {
    fn default() -> Self {
        RemoteServer {
            port: 5174
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

#[tauri::command]
pub fn get_config(config: State<'_, ConfigManager>) -> Config {
    config.config.read().unwrap().clone()
}

#[tauri::command]
pub fn set_config(config: State<'_, ConfigManager>, new_config: Config) {
    *config.config.write().unwrap() = new_config;
    config.save();
}