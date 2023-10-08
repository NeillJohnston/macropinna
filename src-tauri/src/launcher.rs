use crate::config::ConfigManager;

use tauri::State;
use std::{
    collections::BTreeMap,
    process::Child,
    sync::Mutex
};

pub struct LauncherManager {
    fallback_shell: String,
    // TODO this doesn't do much
    children: Mutex<BTreeMap<String, Child>>
}

impl LauncherManager {
    pub fn new() -> Self {
        LauncherManager {
            fallback_shell: "sh".to_string(),
            children: Mutex::new(BTreeMap::new())
        }
    }
}

type LaunchError = String;

#[tauri::command]
pub fn launch(
    manager: State<'_, LauncherManager>,
    config: State<'_, ConfigManager>,
    name: String
) -> Result<(), LaunchError> {
    use std::process::Command;

    let (launcher, shell) = { // Lock for config
        let config = config.config.read().unwrap();

        let launcher = config.launchers
            .iter()
            .find(|launcher| launcher.name == name)
            .ok_or(format!("Could not find launcher \"{}\"", name))?
            .clone();

        let shell = config.shell.clone().unwrap_or_else(|| manager.fallback_shell.clone());

        (launcher, shell)
    };
    
    // Bring already-running processes to the foreground
    #[cfg(unix)]
    {
        let windows = wmctrl::get_windows();

        let window =
            if launcher.finder_is_regex == Some(true) {
                wmctrl::utils::find_window_by_regexp(&windows, &launcher.finder)
            }
            else {
                wmctrl::utils::find_window_by_title(&windows, &launcher.finder)
            };

        if let Some(window) = window {
            window.raise();
            return Ok(());
        }
    }
    #[cfg(windows)]
    {
        // TODO
    }

    let child = Command::new(&shell)
        // TODO this is hardcoded to work with bash/sh, not sure how this goes
        // with things like PowerShell
        .args(["-c", &launcher.command])
        .spawn()
        .map_err(|err| format!("Could not launch: {}", err))?;

    { // Lock for manager.children
        let mut children = manager.children.lock().unwrap();
        children.insert(name, child);
    }

    Ok(())
}