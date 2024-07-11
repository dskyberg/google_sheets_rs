use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

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
        .header("Accept", "application / json")
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
    let response = client
        .post(url)
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application / json")
        .json(payload)
        .send()
        .await?
        .json::<D>()
        .await?;
    Ok(response)
}
