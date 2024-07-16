use anyhow::Result;
use reqwest::Response as ReqwestResponse;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use super::ApiConfig;

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
pub(crate) async fn get<R: DeserializeOwned>(config: &ApiConfig, url: &str) -> Result<R> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(config.headers.clone())
        .send()
        .await?;
    match response.error_for_status_ref() {
        Ok(_res) => Ok(response.json::<R>().await?),
        Err(err) => Err(HttpError::Request(err).into()),
    }
}

/// HTTP PUT that returns the type `T`
///
/// Example:
/// ```ignore
/// let sheet = get::<JsonSheet>(url, access_token).await?;
/// ```
pub(crate) async fn put<P: Serialize, R: DeserializeOwned>(
    config: &ApiConfig,
    url: &str,
    payload: &P,
) -> Result<R> {
    let client = reqwest::Client::new();
    let response = client
        .put(url)
        .headers(config.headers.clone())
        .json(payload)
        .send()
        .await?;
    match response.error_for_status_ref() {
        Ok(_res) => Ok(response.json::<R>().await?),
        Err(err) => Err(HttpError::Request(err).into()),
    }
}

pub(crate) async fn post<RE: Serialize, RO: DeserializeOwned>(
    config: &ApiConfig,
    url: &str,
    payload: &RE,
) -> Result<RO> {
    let client = reqwest::Client::new();
    let response: ReqwestResponse = client
        .post(url)
        .headers(config.headers.clone())
        .json(payload)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_res) => Ok(response.json::<RO>().await?),
        Err(err) => Err(HttpError::Request(err).into()),
    }
}

pub(crate) async fn post_empty<RO: DeserializeOwned>(config: &ApiConfig, url: &str) -> Result<RO> {
    let client = reqwest::Client::new();
    let response: ReqwestResponse = client
        .post(url)
        .headers(config.headers.clone())
        // Google with throw an error if there's no Content-Length header
        .header("Content-Length", "0")
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_res) => Ok(response.json::<RO>().await?),
        Err(err) => Err(HttpError::Request(err).into()),
    }
}
