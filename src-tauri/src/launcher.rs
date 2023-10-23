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

// Bring already-running processes to the foreground
fn raise(finder: &str, is_regex: bool) -> bool {
    #[cfg(unix)]
    {
        let windows = wmctrl::get_windows();

        let window =
            if is_regex {
                wmctrl::utils::find_window_by_regexp(&windows, finder)
            }
            else {
                wmctrl::utils::find_window_by_title(&windows, finder)
            };

        if let Some(window) = window {
            window.raise();
            return true;
        }
    }
    #[cfg(windows)]
    {
        // TODO
    }
    
    return false;
}

pub fn raise_home() {
    // TODO lol more hardcoding
    raise("^Macropinna$", true);
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

        let shell = config.shell
            .clone()
            .unwrap_or_else(|| manager.fallback_shell.clone());

        (launcher, shell)
    };

    if raise(&launcher.finder, launcher.finder_is_regex.unwrap_or(false)) {
        return Ok(());
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