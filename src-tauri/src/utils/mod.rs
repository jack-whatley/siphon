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
