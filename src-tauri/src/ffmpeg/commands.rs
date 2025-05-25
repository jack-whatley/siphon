use crate::installer::RequirementStatus;
use crate::logger::SiphonResult;
use crate::state::AppState;

#[tauri::command]
#[tracing::instrument]
pub async fn ffmpeg_state() -> SiphonResult<RequirementStatus> {
    let app = AppState::get().await?;

    Ok(app.ffmpeg
        .current_status(&app)
        .await?)
}

#[tauri::command]
#[tracing::instrument]
pub async fn update_ffmpeg() -> SiphonResult<()> {
    let app = AppState::get().await?;

    Ok(app.ffmpeg
        .update(&app)
        .await?)
}
