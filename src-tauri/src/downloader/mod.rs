use crate::downloader::install::VersionWrapper;
use eyre::Result;
use async_trait::async_trait;
use crate::installer::{RequirementInstaller, RequirementStatus};
use crate::state::AppState;

pub mod commands;
pub mod install;
pub mod runner;

pub struct DownloaderRequirement;

#[async_trait]
impl RequirementInstaller for DownloaderRequirement {
    async fn current_status(&self, app: &AppState) -> Result<RequirementStatus> {
        let disk_version = runner::get_version().await;

        if disk_version.is_none() {
            return Ok(RequirementStatus::Missing);
        }

        let has_update = check_for_update(
            disk_version.unwrap(),
            app
        ).await?;

        if has_update {
            Ok(RequirementStatus::Update)
        } else {
            Ok(RequirementStatus::Installed)
        }
    }

    async fn update(&self, app: &AppState) -> Result<()> {
        let current_status = self.current_status(app).await?;

        if current_status == RequirementStatus::Installed {
            tracing::info!("Downloader is already installed, exiting download");

            return Ok(());
        }

        install::download_latest_version(app).await?;

        Ok(())
    }
}

async fn check_for_update(current: VersionWrapper, app: &AppState) -> Result<bool> {
    let latest_version = install::fetch_latest_version(app).await?;

    Ok(latest_version > current)
}
