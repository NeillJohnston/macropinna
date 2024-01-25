use notify::RecommendedWatcher;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tauri::State;
use serde::Serialize;

use crate::GlobalAppHandle;
use shared::{
    config::{self, Config},
    util::project_dirs::PROJECT_DIRS
};

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

                config::save_to_path(&path, &config);

                config
            }
            else {
                config::load_from_path(&path).unwrap()
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
                        if let Some(config) = config::load_from_path(&path) {
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
        config::save_to_path(&self.path, &self.config.read().unwrap());
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