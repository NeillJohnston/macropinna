use shared::config::*;

#[tauri::command]
pub fn suggest_launchers() -> Vec<Launcher> {
    use std::process::Command;

    struct FindLauncher {
        checker: &'static str,
        name: &'static str,
        command: &'static str,
        finder: &'static str,
        finder_is_regex: bool,
        colors: [&'static str; 3]
    }

    // TODO find a way to move this to a separate (data) file - may involve an
    // extra build step
    let find_launchers = [
        // Media
        FindLauncher {
            checker: "which spotify",
            name: "Spotify",
            command: "spotify",
            finder: "Spotify",
            finder_is_regex: false,
            colors: ["#23cf5f", "#0c4620", "#030e06"]
        },
        FindLauncher {
            checker: "which kodi",
            name: "Kodi",
            command: "kodi",
            finder: "Kodi",
            finder_is_regex: false,
            colors: ["#12b2e7", "#063646", "#000000"]
        },
        // Browsing
        FindLauncher {
            checker: "which firefox",
            name: "Firefox",
            command: "firefox",
            finder: "Mozilla Firefox",
            finder_is_regex: false,
            colors: ["#ffa048", "#e06c4d", "#a40757"]
        },
        FindLauncher {
            checker: "which firefox",
            name: "Reddit",
            command: "firefox --new-window \"https://www.reddit.com/\"",
            finder: "Reddit",
            finder_is_regex: false,
            colors: ["#ffffff", "#ffbfa8", "#ff4300"]
        },
        FindLauncher {
            checker: "which firefox",
            name: "YouTube",
            command: "firefox --new-window \"https://www.youtube.com/\"",
            finder: "YouTube",
            finder_is_regex: false,
            colors: ["#ffffff", "#ffadad", "#ff0000"]
        },
        // Admin/other
        FindLauncher {
            checker: "which gnome-terminal",
            name: "Terminal",
            command: "gnome-terminal",
            finder: "Terminal",
            finder_is_regex: false,
            colors: ["#dfdedb", "#4d4857", "#221c30"]
        },
        FindLauncher {
            checker: "which xfce4-terminal",
            name: "Terminal",
            command: "xfce4-terminal",
            finder: "Terminal",
            finder_is_regex: false,
            colors: ["#dfdedb", "#4d4857", "#221c30"]
        },
    ];

    find_launchers
        .into_iter()
        .filter(|find_launcher| {
            Command::new("sh")
                .args(["-c", find_launcher.checker])
                .output()
                .is_ok_and(|output| output.status.success())
        })
        .map(|find_launcher| {
            Launcher {
                name: find_launcher.name.to_string(),
                command: find_launcher.command.to_string(),
                finder: find_launcher.finder.to_string(),
                finder_is_regex: if find_launcher.finder_is_regex { Some(true) } else { None },
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