use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
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
