use crate::config::Launcher;

#[tauri::command]
pub fn suggest_launchers() -> Vec<Launcher> {
    use std::process::Command;

    struct FindLauncher {
        checker: &'static str,
        name: &'static str,
        command: &'static str,
        finder: &'static str,
        finder_is_regex: Option<bool>,
        colors: [&'static str; 3]
    }

    let find_launchers = [
        FindLauncher {
            checker: "which spotify",
            name: "Spotify",
            command: "spotify",
            finder: "Spotify",
            finder_is_regex: None,
            colors: ["#23cf5f", "#0c4620", "#030e06"]
        },
        FindLauncher {
            checker: "which xdg-open",
            name: "YouTube",
            command: "xdg-open \"https://www.youtube.com/\"",
            finder: "YouTube",
            finder_is_regex: None,
            colors: ["#ffffff", "#ffadad", "#ff0000"]
        },
        FindLauncher {
            checker: "which xdg-open",
            name: "Reddit",
            command: "xdg-open \"https://www.reddit.com/\"",
            finder: "Reddit",
            finder_is_regex: None,
            colors: ["#ffffff", "#ffbfa8", "#ff4300"]
        },
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
                image_path: None,
                css_background: Some(format!(
                    "linear-gradient(105deg, {} 0%, {} 50%, {} 100%)",
                    find_launcher.colors[0],
                    find_launcher.colors[1],
                    find_launcher.colors[2],
                ))
            }
        })
        .collect()
}