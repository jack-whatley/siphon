use async_trait::async_trait;
use eyre::Result;
use serde::{Deserialize, Serialize};
use crate::downloader::DownloaderRequirement;
use crate::ffmpeg::FFMPEGRequirement;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RequirementStatus {
    Missing,
    Update,
    Installed,
}

#[async_trait]
pub trait RequirementInstaller {
    /// Fetches the current status of the requirement.
    async fn current_status(&self, app: &AppState) -> Result<RequirementStatus>;

    /// Installs or updates the requirement.
    async fn update(&self, app: &AppState) -> Result<()>;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Requirements {
    Downloader,
    FFMPEG,
}

impl Requirements {
    pub fn get_latest_url(&self) -> String {
        match self {
            Requirements::Downloader => "https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest".to_owned(),
            Requirements::FFMPEG => "https://api.github.com/repos/GyanD/codexffmpeg/releases/latest".to_owned(),
        }
    }
}

pub fn get_requirement_handler(req: Requirements) -> Box<dyn RequirementInstaller + Send + Sync> {
    match req {
        Requirements::Downloader => Box::new(DownloaderRequirement),
        Requirements::FFMPEG => Box::new(FFMPEGRequirement),
    }
}
