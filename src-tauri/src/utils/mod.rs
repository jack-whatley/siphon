use std::path::PathBuf;
use bytes::Bytes;
use eyre::Result;
use futures::StreamExt;

pub mod net;
pub mod paths;

pub async fn write_stream_to_file(
    file: &mut tokio::fs::File,
    bytes: &mut (impl futures::Stream<Item = Result<Bytes, reqwest::Error>> + Unpin),
) -> Result<()> {
    while let Some(chunk) = bytes.next().await {
        let chunk = chunk?;

        tokio::io::copy(&mut chunk.as_ref(), file).await?;
    }

    Ok(())
}

/// Iterates through a directory, returning the full path to each
/// item in it.
pub async fn iterate_directory(path: impl Into<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];
    let mut items = tokio::fs::read_dir(path.into()).await?;

    while let Some(entry) = items.next_entry().await? {
        paths.push(entry.path());

        if entry.path().is_dir() {
            paths.append(&mut Box::pin(iterate_directory(entry.path().clone())).await?)
        }
    }

    Ok(paths)
}
