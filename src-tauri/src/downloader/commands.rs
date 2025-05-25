use crate::downloader::runner;
use crate::logger::{send_ui_event, SiphonResult, UIEvent};
use crate::state::AppState;
use eyre::OptionExt;
use std::path::PathBuf;
use crate::installer::RequirementStatus;

#[tauri::command]
#[tracing::instrument]
pub async fn downloader_state() -> SiphonResult<RequirementStatus> {
    let app = AppState::get().await?;

    Ok(app.downloader
        .current_status(&app)
        .await?)
}

#[tauri::command]
#[tracing::instrument]
pub async fn update_downloader() -> SiphonResult<()> {
    let app = AppState::get().await?;

    Ok(app.downloader
        .update(&app)
        .await?)
}

#[tauri::command]
#[tracing::instrument]
pub fn default_download_dir() -> SiphonResult<String> {
    Ok(dirs_next::desktop_dir()
        .ok_or_eyre("Failed to locate computer desktop directory")?
        .display()
        .to_string())
}

#[tauri::command]
#[tracing::instrument]
pub fn all_presets() -> Vec<String> {
    runner::PresetTypes::all()
}

#[tauri::command]
#[tracing::instrument(skip(app))]
pub async fn download_video(
    url: String,
    directory: String,
    preset: runner::PresetTypes,
    app: tauri::AppHandle,
) -> SiphonResult<String> {
    let std_out = runner::CommandBuilder::new()
        .with_ffmpeg_args()
        .with_download_path(PathBuf::from(&directory))
        .with_preset(preset)
        .with_url(url)
        .run()
        .await?;

    let download_path = runner::extract_download_path(&directory, &std_out, preset);
    send_ui_event(UIEvent::downloaded_video(&download_path), &app);

    Ok(download_path)
}
