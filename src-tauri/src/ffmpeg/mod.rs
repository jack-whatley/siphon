use crate::utils;
use crate::utils::net;
use eyre::Result;

pub const FFMPEG_LINK: &str = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-git-essentials.7z";

pub async fn download_specific_version(client: &reqwest::Client) -> Result<()> {
    let ffmpeg_path = utils::paths::ffmpeg_executable();

    if ffmpeg_path.exists() {
        tracing::info!("ffmpeg is already installed, skipping installation");
        return Ok(());
    }

    let mut download_stream = net::fetch_stream(client, FFMPEG_LINK, None).await?;

    let mut file_handle = tokio::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(utils::paths::default_app_dir().join("ffmpeg-git-essentials.7z"))
        .await?;

    utils::write_stream_to_file(&mut file_handle, &mut download_stream).await?;

    drop(file_handle);

    tokio::task::spawn_blocking(move || -> Result<()> {
        let path = utils::paths::default_app_dir().join("ffmpeg-git-essentials.7z");
        sevenz_rust::decompress_file(path, utils::paths::ffmpeg_dir())?;

        Ok(())
    })
    .await??;

    tokio::fs::remove_file(utils::paths::default_app_dir().join("ffmpeg-git-essentials.7z"))
        .await?;

    Ok(())
}
