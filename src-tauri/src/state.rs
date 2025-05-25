use crate::installer::{get_requirement_handler, RequirementInstaller, Requirements};
use crate::utils::{net, paths};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
use crate::github::GithubCache;

static SIPHON_APP: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub struct AppState {
    pub http: reqwest::Client,
    pub downloader: Box<dyn RequirementInstaller + Send + Sync>,
    pub ffmpeg: Box<dyn RequirementInstaller + Send + Sync>,
    pub github: Mutex<GithubCache>,
}

impl AppState {
    pub async fn get() -> Result<Arc<Self>> {
        Ok(Arc::clone(SIPHON_APP.get_or_try_init(Self::init).await?))
    }

    async fn init() -> Result<Arc<Self>> {
        let http = net::client()?;
        let downloader = get_requirement_handler(Requirements::Downloader);
        let ffmpeg = get_requirement_handler(Requirements::FFMPEG);
        let github = Mutex::new(GithubCache::init().await?);

        let dir_path = paths::default_app_dir();

        if !dir_path.exists() {
            tokio::fs::create_dir_all(dir_path).await?;
        }

        let state = AppState { http, downloader, ffmpeg, github };

        Ok(Arc::new(state))
    }
}
