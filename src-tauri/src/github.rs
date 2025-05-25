use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use crate::installer::Requirements;
use crate::utils;
use crate::utils::net;

const CACHE_REFRESH_AGE: i64 = 3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseCache {
    fetch_date: DateTime<Utc>,
    release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCache {
    caches: HashMap<Requirements, ReleaseCache>,
}

impl GithubCache {
    pub async fn init() -> eyre::Result<Self> {
        let cache_path = utils::paths::cache_path();

        if cache_path.exists() {
            Ok(Self::load_from_file(&cache_path).await?)
        } else {
            Ok(Self { caches: HashMap::new() })
        }
    }

    pub async fn fetch_latest(&mut self, client: &reqwest::Client, req: Requirements) -> eyre::Result<Release> {
        if self.req_in_map(&req) {
            let cached = &self.caches[&req];
            let current_time = Utc::now();
            let diff = current_time - cached.fetch_date;

            if diff.num_days() >= CACHE_REFRESH_AGE {
                tracing::info!(
                    "Release Cache {:#?} is out of date, updating",
                    req
                );

                Ok(self.fetch_latest_inner(client, req).await?)
            }
            else {
                tracing::info!(
                    "Retrieving release cache for {:#?}",
                    req
                );

                Ok(cached.release.clone())
            }
        }
        else {
            tracing::info!(
                "Release Cache {:#?} missing, updating",
                req
            );

            Ok(self.fetch_latest_inner(client, req).await?)
        }
    }

    /// Fetches the latest version from github directly, ignoring any
    /// cached values.
    async fn fetch_latest_inner(&mut self, client: &reqwest::Client, req: Requirements) -> eyre::Result<Release> {
        let url = req.get_latest_url();

        let latest_release = net::fetch_json::<Release>(
            client,
            &url,
            None,
        ).await?;

        self.caches.entry(req)
            .and_modify(|x| {
                x.release = latest_release.clone();
                x.fetch_date = Utc::now();
            })
            .or_insert(ReleaseCache {
                fetch_date: Utc::now(),
                release: latest_release.clone(),
            });

        self.save_to_file().await?;

        Ok(latest_release)
    }

    async fn save_to_file(&self) -> eyre::Result<()> {
        let serialized = serde_json::to_vec(&self)?;

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(utils::paths::cache_path())
            .await?;

        file.write_all(&serialized).await?;

        Ok(())
    }

    async fn load_from_file(path: &Path) -> eyre::Result<Self> {
        let file_data = tokio::fs::read(path).await?;

        Ok(serde_json::from_slice(&file_data)?)
    }

    fn req_in_map(&self, req: &Requirements) -> bool {
        self.caches.keys().find(|x| x == &req).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::Release;
    use crate::utils::net;

    #[tokio::test]
    async fn fetch_github_release() {
        let client = net::client().unwrap();

        let release: Release = net::fetch_json(
            &client,
            "https://api.github.com/repos/yt-dlp/yt-dlp/releases/215902436",
            None,
        )
            .await
            .unwrap();

        assert_eq!(&release.name, "yt-dlp 2025.04.30");
        assert_eq!(&release.tag_name, "2025.04.30");
        assert_eq!(release.assets.len(), 16);
    }
}