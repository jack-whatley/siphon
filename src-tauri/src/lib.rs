use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

mod downloader;
mod ffmpeg;
mod logger;
mod state;
mod utils;
mod installer;
mod github;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init().unwrap_or_else(|err| eprintln!("Failed to setup logger: {:#?}", err));

    tauri::Builder::default()
        .setup(|app| {
            app.handle().plugin(tauri_plugin_dialog::init())
                .expect("Failed to initialise dialog");

            app.handle().plugin(tauri_plugin_updater::Builder::new().build())
                .expect("Failed to initialise updater");

            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                update(handle).await
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            logger::open_log,
            logger::log_error,
            downloader::commands::downloader_state,
            downloader::commands::update_downloader,
            downloader::commands::default_download_dir,
            downloader::commands::all_presets,
            downloader::commands::download_video,
            ffmpeg::commands::ffmpeg_state,
            ffmpeg::commands::update_ffmpeg,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Siphon Application...");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        let do_update: bool = app.dialog()
            .message(format!("There is a new release, version: '{:?}'. Do you wish to update now?", update.version))
            .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
            .title("Application Update Detected")
            .blocking_show();

        if do_update {
            update.download_and_install(
                |chunk_length, _content_length| {
                    downloaded += chunk_length;
                },
                || {
                    println!("download finished");
                }
            ).await?;

            let _message = app.dialog()
                .message("The application has successfully updated. It will now restart...")
                .kind(MessageDialogKind::Warning)
                .title("Application Update Complete")
                .blocking_show();

            app.restart();
        }
    }

    Ok(())
}
