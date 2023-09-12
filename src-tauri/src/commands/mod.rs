use std::process::Command;

#[tauri::command]
// TODO return Result
pub async fn keystone_correct(_angle: f64) {
    let (a, b, c, d, e, f, g, h, i) = (0, 0, 0, 0, 0, 0, 0, 0, 0);

    let _ = Command::new("xrandr")
        .arg("--transform")
        .arg(format!("{a},{b},{c},{d},{e},{f},{g},{h},{i}"))
        .output();
}

// #[tauri::command]
// pub async fn get_weather() {
//     let res = reqwest::get(url)
//         .await?
//         .text()
//         .await?;
// }