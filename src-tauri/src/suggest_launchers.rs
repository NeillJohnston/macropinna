use crate::config::Launcher;

#[tauri::command]
pub fn suggest_launchers() -> Vec<Launcher> {
    use std::process::Command;

    struct FindLauncher {
        checker: &'static str,
        name: &'static str,
        command: &'static str,
        finder: &'static str,
        finder_is_regex: Option<bool>
    }

    let find_launchers = [
        FindLauncher {
            checker: "which spotify",
            name: "Spotify",
            command: "spotify",
            finder: "Spotify",
            finder_is_regex: None
        }
    ];

    find_launchers
        .into_iter()
        .filter(|find_launcher| {
            Command::new("sh")
                .args(["-c", find_launcher.checker])
                .output()
                .is_ok()
        })
        .map(|find_launcher| {
            Launcher {
                name: find_launcher.name.to_string(),
                command: find_launcher.command.to_string(),
                finder: find_launcher.finder.to_string(),
                finder_is_regex: find_launcher.finder_is_regex,
                image_path: None
            }
        })
        .collect()
}