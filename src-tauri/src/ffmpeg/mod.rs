use async_trait::async_trait;
use eyre::{Context, OptionExt};
use crate::ffmpeg::version_store::Version;
use crate::github::Release;
use crate::installer::{RequirementInstaller, RequirementStatus, Requirements};
use crate::state::AppState;
use crate::utils;
use crate::utils::net;

pub mod runner;
mod version_store;
pub mod commands;

pub struct FFMPEGRequirement;

#[async_trait]
impl RequirementInstaller for FFMPEGRequirement {
    async fn current_status(&self, app: &AppState) -> eyre::Result<RequirementStatus> {
        let ffmpeg_path = utils::paths::ffmpeg_executable();

        if !ffmpeg_path.exists() {
            return Ok(RequirementStatus::Missing);
        }

        let disk_version = version_store::get().await;

        match disk_version {
            Some(version) => {
                let latest_release = latest_release(&app).await?;
                let latest_version = latest_release.tag_name;

                if latest_version != version.version {
                    Ok(RequirementStatus::Update)
                }
                else {
                    Ok(RequirementStatus::Installed)
                }
            }
            None => {
                tracing::info!("FFMPEG versioning file is missing, assuming ffmpeg missing");

                Ok(RequirementStatus::Missing)
            }
        }
    }

    async fn update(&self, app: &AppState) -> eyre::Result<()> {
        let current_status = self.current_status(app).await?;

        if current_status == RequirementStatus::Installed {
            tracing::info!("FFMPEG is already installed, exiting download early");

            return Ok(());
        }

        download_latest_ffmpeg(app).await?;

        Ok(())
    }
}

async fn latest_release(app: &AppState) -> eyre::Result<Release> {
    app.github
        .lock()
        .await
        .fetch_latest(&app.http, Requirements::FFMPEG)
        .await
}

const FFMPEG_RELEASE_PATTERN: &str = "-essentials_build.7z";

async fn download_latest_ffmpeg(app: &AppState) -> eyre::Result<()> {
    cleanup_files(true).await?;

    let latest = latest_release(app).await?;

    let download_url = latest.assets
        .iter()
        .find(|x| x.name.ends_with(FFMPEG_RELEASE_PATTERN))
        .ok_or_eyre("Failed to find FFMPEG release asset on github")?
        .browser_download_url
        .as_str();

    let mut download_stream = net::fetch_stream(&app.http, download_url, None)
        .await
        .wrap_err_with(|| format!("Failed to download asset {}", download_url))?;

    let mut file_handle = tokio::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(utils::paths::default_app_dir().join("ffmpeg.7z"))
        .await?;

    utils::write_stream_to_file(&mut file_handle, &mut download_stream).await?;

    drop(file_handle);

    tokio::task::spawn_blocking(move || -> eyre::Result<()> {
        let src_path = utils::paths::default_app_dir().join("ffmpeg.7z");
        let output_dir = utils::paths::ffmpeg_download_dir();

        sevenz_rust::decompress_file(
            &src_path,
            &output_dir,
        )?;

        Ok(())
    }).await??;

    let all_ffmpeg_paths = utils::iterate_directory(utils::paths::ffmpeg_download_dir()).await?;

    for path in all_ffmpeg_paths {
        if path.ends_with("ffmpeg.exe") {
            tokio::fs::copy(&path, utils::paths::ffmpeg_executable()).await?;
            update_disk_version(&latest).await?;
            cleanup_files(false).await?;
        }
    }

    Ok(())
}

async fn cleanup_files(including_executable: bool) -> eyre::Result<()> {
    let ffmpeg_zip = utils::paths::default_app_dir().join("ffmpeg.7z");
    let ffmpeg_download = utils::paths::ffmpeg_download_dir();
    let ffmpeg_executable = utils::paths::ffmpeg_executable();
    let old_dir = utils::paths::default_app_dir().join("ffmpeg");

    if ffmpeg_zip.exists() {
        tokio::fs::remove_file(&ffmpeg_zip).await?;
    }

    if ffmpeg_download.exists() {
        tokio::fs::remove_dir_all(&ffmpeg_download).await?;
    }

    if including_executable && ffmpeg_executable.exists() {
        tokio::fs::remove_file(&ffmpeg_executable).await?;
    }

    if old_dir.exists() && old_dir.is_dir() {
        tokio::fs::remove_dir_all(&old_dir).await?;
    }

    Ok(())
}

async fn update_disk_version(latest: &Release) -> eyre::Result<()> {
    let version = Version { version: latest.tag_name.clone() };

    version_store::set(version).await?;

    Ok(())
}
