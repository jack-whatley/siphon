use crate::downloader::Downloader;
use crate::utils::{net, paths};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

static SIPHON_APP: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub struct AppState {
    pub http: reqwest::Client,
    pub downloader: Mutex<Downloader>,
}

impl AppState {
    pub async fn get() -> Result<Arc<Self>> {
        Ok(Arc::clone(SIPHON_APP.get_or_try_init(Self::init).await?))
    }

    pub async fn lock_downloader(&self) -> MutexGuard<Downloader> {
        self.downloader.lock().await
    }

    async fn init() -> Result<Arc<Self>> {
        let http = net::client()?;
        let downloader = Mutex::new(Downloader::init().await?);

        let dir_path = paths::default_app_dir();

        if !dir_path.exists() {
            tokio::fs::create_dir_all(dir_path).await?;
        }

        let state = AppState { http, downloader };

        Ok(Arc::new(state))
    }
}
