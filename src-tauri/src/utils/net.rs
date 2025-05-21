use bytes::Bytes;
use eyre::{bail, Context, Result};
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::time::Duration;

pub fn client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .user_agent("dev.jackwhatley.siphon")
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .build()
        .with_context(|| "Failed to create http client for app state".to_string())?)
}

pub async fn fetch_url(
    client: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    retry_attempts: Option<u32>,
) -> Result<Response> {
    let retry_attempts = retry_attempts.unwrap_or(3);

    for i in 0..=retry_attempts {
        let request = client.request(method.clone(), url);
        let result = request.send().await;

        match result {
            Ok(response) => {
                if !response.status().is_success() {
                    if i <= retry_attempts {
                        tracing::warn!(
                            "Received unsuccessful response, retrying attempt {}/{}: {:#?}",
                            i,
                            retry_attempts,
                            response
                        );

                        continue;
                    } else {
                        tracing::error!(
                            "Received unsuccessful response after using all retry attempts."
                        );
                        bail!("Received unsuccessful response after using all retry attempts.");
                    }
                }

                return Ok(response);
            }
            Err(error) if i <= retry_attempts => {
                tracing::warn!(
                    "Request failed on retry attempt {}/{}, retrying: {:#?}",
                    i,
                    retry_attempts,
                    error
                );

                continue;
            }
            Err(error) => {
                tracing::error!("Request failed after all attempts: {:#?}", error);
                bail!("Request failed after all attempts: {:#?}", error);
            }
        }
    }

    unreachable!()
}

pub async fn fetch_json<T>(
    client: &reqwest::Client,
    url: &str,
    retry_attempts: Option<u32>,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let response = fetch_url(client, reqwest::Method::GET, url, retry_attempts).await?;

    Ok(response.json().await?)
}

pub async fn fetch_stream(
    client: &reqwest::Client,
    url: &str,
    retry_attempts: Option<u32>,
) -> Result<impl futures::stream::Stream<Item = Result<Bytes, reqwest::Error>> + Unpin> {
    let response = fetch_url(client, reqwest::Method::GET, url, retry_attempts).await?;

    Ok(response.bytes_stream())
}

#[cfg(test)]
mod tests {
    use super::{client, fetch_json};
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Todo {
        pub user_id: i64,
        pub id: i64,
        pub title: String,
        pub completed: bool,
    }

    #[tokio::test]
    async fn test_query_json() {
        let client = client().unwrap();

        let todo = fetch_json::<Todo>(
            &client,
            "https://jsonplaceholder.typicode.com/todos/1",
            None,
        )
        .await
        .unwrap();

        assert_eq!(todo.user_id, 1);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "delectus aut autem".to_owned());
        assert_eq!(todo.completed, false);
    }
}
