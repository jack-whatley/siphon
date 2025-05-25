use crate::utils;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
}

#[tracing::instrument]
pub async fn get() -> Option<Version> {
    let version_path = utils::paths::ffmpeg_version();

    if !version_path.exists() {
        tracing::info!("FFMPEG version file doesn't exist, returning None");
        return None;
    }

    let file_data = tokio::fs::read(&version_path).await.ok()?;

    Some(serde_json::from_slice(&file_data).ok()?)
}

#[tracing::instrument]
pub async fn set(version: Version) -> eyre::Result<()> {
    let serialised = serde_json::to_vec_pretty(&version)?;

    let mut file = tokio::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(utils::paths::ffmpeg_version())
        .await?;

    file.write_all(serialised.as_slice()).await?;

    Ok(())
}
