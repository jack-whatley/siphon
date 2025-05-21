use crate::downloader::install::VersionWrapper;
use crate::logger::UIEvent;
use crate::{logger, utils};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;

pub mod commands;
pub mod install;
pub mod models;
pub mod runner;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum InstallStatus {
    Missing,
    UpdateAvailable,
    Installed,
}

pub struct Downloader {
    version: Mutex<Option<VersionWrapper>>,
    update_available: AtomicBool,
}

impl Downloader {
    pub async fn init() -> Result<Self> {
        let version = runner::get_version().await;

        Ok(Self {
            version: Mutex::new(version),
            update_available: AtomicBool::new(false),
        })
    }

    pub async fn current_status(
        &mut self,
        client: &reqwest::Client,
        app: tauri::AppHandle,
    ) -> Result<InstallStatus> {
        let disk_version = runner::get_version().await;

        if disk_version.is_none() {
            logger::send_ui_event(UIEvent::downloader_missing(), &app);
            return Ok(InstallStatus::Missing);
        }

        let has_update = self.check_for_update(client).await?;

        if has_update {
            logger::send_ui_event(UIEvent::downloader_update_available(), &app);
            Ok(InstallStatus::UpdateAvailable)
        } else {
            let ffmpeg_path = utils::paths::ffmpeg_executable();
            if !ffmpeg_path.exists() {
                return Ok(InstallStatus::Missing);
            }

            Ok(InstallStatus::Installed)
        }
    }

    pub async fn update(&mut self, client: &reqwest::Client, app: tauri::AppHandle) -> Result<()> {
        let current_status = self.current_status(client, app.clone()).await?;

        if current_status == InstallStatus::Installed {
            tracing::info!(
                "Install Status is already {:?}, exiting download",
                current_status
            );
            return Ok(());
        }

        install::download_latest_version(client).await?;
        logger::send_ui_event(UIEvent::downloader_installed(), &app);

        Ok(())
    }

    async fn check_for_update(&mut self, client: &reqwest::Client) -> Result<bool> {
        let latest_version = install::fetch_latest_version(client).await?;
        let current_version = self.version.lock().await;

        match *current_version {
            Some(ref version) => {
                let result = latest_version > *version;

                self.update_available = AtomicBool::new(result);
                Ok(result)
            }
            None => {
                self.update_available = AtomicBool::new(true);
                Ok(true)
            }
        }
    }
}
