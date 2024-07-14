use anyhow::Result;
use reqwest::Response as ReqwestResponse;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request error")]
    Request(#[from] reqwest::Error),
}

/// HTTP Get that returns the type `T`
///
/// Example:
/// ```ignore
/// let sheet = get::<JsonSheet>(url, access_token).await?;
/// ```
pub(crate) async fn get<T: DeserializeOwned>(url: &str, token: &str) -> Result<T> {
    let client = reqwest::Client::new();
    let value = client
        .get(url)
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(value)
}

pub(crate) async fn put<T: Serialize>(url: &str, token: &str, payload: &T) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .put(url)
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application / json")
        .json(payload)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        println!("{}", status.canonical_reason().unwrap());
        let json = response.json::<serde_json::Value>().await?;
        println!("{:?}", json);
    }
    Ok(())
}

pub(crate) async fn post<T: Serialize, D: DeserializeOwned>(
    url: &str,
    token: &str,
    payload: &T,
) -> Result<D> {
    let client = reqwest::Client::new();
    let response: ReqwestResponse = client
        .post(url)
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application / json")
        .json(payload)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_res) => Ok(response.json::<D>().await?),
        Err(err) => Err(HttpError::Request(err).into()),
    }
}
