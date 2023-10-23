#[derive(serde::Serialize)]
pub struct PlayerMetadataResponse {
    album_name: Option<String>,
    disc_number: Option<i32>,
    track_title: Option<String>,
    track_number: Option<i32>,
    track_artists: Option<Vec<String>>,
    track_length_us: Option<u64>,
    art_url: Option<String>,
}

#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

#[cfg(unix)]
mod unix {
    use super::*;

    #[tauri::command]
    pub fn get_player_metadata() -> Option<PlayerMetadataResponse> {
        use mpris::PlayerFinder;
    
        let player = PlayerFinder::new()
            .ok()?
            .find_active()
            .ok()?;
    
        let metadata = player
            .get_metadata()
            .ok()?;
    
        Some(PlayerMetadataResponse {
            album_name: metadata.album_name().map(|s| s.into()),
            disc_number: metadata.disc_number(),
            track_title: metadata.title().map(|s| s.into()),
            track_number: metadata.track_number(),
            track_artists: metadata.artists().map(|v| v.into_iter().map(|s| s.into()).collect()),
            track_length_us: metadata.length_in_microseconds(),
            art_url: metadata.art_url().map(|s| s.into()),
        })
    }
}

#[cfg(windows)]
mod windows {
    use super::*;
    use tauri::State;

    pub struct MediaPlayerManager;

    #[tauri::command]
    pub fn get_player_metadata(_media_player: State<'_, MediaPlayerManager>) -> Option<PlayerMetadataResponse> {
        // TODO
        None
    }
}