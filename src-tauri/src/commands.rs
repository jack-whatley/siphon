use crate::installer::RequirementStatus;
use crate::logger::{logger_path, SiphonResult};
use crate::state::AppState;

#[tauri::command]
pub fn open_log() -> std::result::Result<(), String> {
    let log_path = logger_path();

    tauri_plugin_opener::open_path(&log_path, None::<&str>)
        .map_err(|err| format!("Failed to open log file: {:#?}", err))?;

    Ok(())
}

#[tauri::command]
pub fn log_error(msg: String) {
    tracing::error!("{}", msg);
}

#[tauri::command]
pub async fn initial_setup_required() -> SiphonResult<bool> {
    let app = AppState::get().await?;
    let downloader = app.downloader.current_status(&app).await?;
    let ffmpeg = app.ffmpeg.current_status(&app).await?;

    Ok(downloader == RequirementStatus::Missing && ffmpeg == RequirementStatus::Missing)
}
