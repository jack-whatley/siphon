use crate::utils;
use crate::utils::net;
use eyre::{ensure, Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use crate::github::Release;
use crate::installer::Requirements;
use crate::state::AppState;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VersionWrapper {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl VersionWrapper {
    pub fn parse(version: &str) -> Result<Self> {
        let split = version.split('.').collect::<Vec<&str>>();

        ensure!(
            split.len() == 3,
            "Version format should contain three sections split by the . character"
        );

        let year = split[0]
            .parse::<u32>()
            .wrap_err(format!("Failed to parse version wrapper year {}", split[0]))?;
        let month = split[1].parse::<u32>().wrap_err(format!(
            "Failed to parse version wrapper month {}",
            split[1]
        ))?;
        let day = split[2]
            .parse::<u32>()
            .wrap_err(format!("Failed to parse version wrapper day {}", split[2]))?;

        Ok(Self { year, month, day })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.year, self.month, self.day)
    }
}

pub async fn fetch_latest_version(app: &AppState) -> Result<VersionWrapper> {
    let latest_release = latest_release(app).await?;

    Ok(VersionWrapper::parse(&latest_release.tag_name)?)
}

pub async fn download_latest_version(app: &AppState) -> Result<()> {
    let latest_release = latest_release(app).await?;

    let download_url = latest_release
        .assets
        .iter()
        .find(|x| x.name == utils::paths::DOWNLOAD_EXECUTABLE)
        .ok_or_eyre("Failed to find release that contains correct executable.")?
        .browser_download_url
        .as_str();

    let mut download_stream = net::fetch_stream(&app.http, download_url, None)
        .await
        .wrap_err_with(|| format!("Failed to download asset {}", download_url))?;

    let mut file_handle = tokio::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(utils::paths::downloader_path())
        .await?;

    utils::write_stream_to_file(&mut file_handle, &mut download_stream).await?;

    Ok(())
}

async fn latest_release(app: &AppState) -> Result<Release> {
    app.github
        .lock()
        .await
        .fetch_latest(&app.http, Requirements::Downloader)
        .await
}

#[cfg(test)]
mod tests {
    use crate::downloader::install::VersionWrapper;

    #[test]
    fn parse_valid_version() {
        let version_str = "2025.04.30";
        let wrapper = VersionWrapper::parse(version_str).unwrap();

        assert_eq!(wrapper.year, 2025);
        assert_eq!(wrapper.month, 4);
        assert_eq!(wrapper.day, 30);
    }

    #[test]
    fn parse_invalid_long_version() {
        let version_str = "2025.04.30.99";
        let wrapper = VersionWrapper::parse(version_str);

        assert!(wrapper.is_err());
    }

    #[test]
    fn parse_invalid_characters_version() {
        let version_str = "20aa.04.b0";
        let wrapper = VersionWrapper::parse(version_str);

        assert!(wrapper.is_err());
    }
}
