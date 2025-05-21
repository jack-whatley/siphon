use crate::downloader::{runner, InstallStatus};
use crate::logger::{send_ui_event, SiphonResult, UIEvent};
use crate::state::AppState;
use eyre::OptionExt;
use std::path::PathBuf;

#[tauri::command]
pub async fn downloader_state(app: tauri::AppHandle) -> SiphonResult<InstallStatus> {
    let state = AppState::get().await?;
    let mut downloader = state.lock_downloader().await;

    Ok(downloader.current_status(&state.http, app).await?)
}

#[tauri::command]
pub async fn update_downloader(app: tauri::AppHandle) -> SiphonResult<()> {
    let state = AppState::get().await?;
    let mut downloader = state.lock_downloader().await;

    crate::ffmpeg::download_specific_version(&state.http).await?;

    Ok(downloader.update(&state.http, app).await?)
}

#[tauri::command]
pub fn default_download_dir() -> SiphonResult<String> {
    Ok(dirs_next::desktop_dir()
        .ok_or_eyre("Failed to locate computer desktop directory")?
        .display()
        .to_string())
}

#[tauri::command]
pub fn all_presets() -> Vec<String> {
    runner::PresetTypes::all()
}

#[tauri::command]
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
