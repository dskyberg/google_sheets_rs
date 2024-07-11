use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::Error;

use super::source::ApplicationDefaultCredentials;

const DEFAULT_USER_GRANT: &str = "refresh_token";
const GOOGLE_OAUTH2_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

/// The request body for talking to `https://oauth2.googleapis.com/token`.
#[derive(Serialize)]
struct UserTokenRequest<'a> {
    grant_type: &'a str,
    refresh_token: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
}

/// The response of a Service Account Key token exchange.
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    id_token: Option<String>,
    expires_in: i64,
}

/// AccessToken holds a token value that can be used in Authorization headers to
/// authenticate with Google Cloud APIs.
#[derive(Debug, Clone)]
pub struct AccessToken {
    /// The actual token.
    pub value: String,
    #[allow(dead_code)]
    expires: Option<DateTime<Utc>>,
}

impl AccessToken {
    /// Returns true if the token should be considered valid, compensating for
    /// clock skew with by ten seconds.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        if let Some(expires) = self.expires {
            let now = Utc::now();
            // Avoid clock skew with 10 second diff of now.
            let expiresish = expires - Duration::seconds(10);
            expiresish > now
        } else {
            false
        }
    }
}

/// Retrieves an [AccessToken] based on configured source.
pub async fn fetch_access_token() -> Result<AccessToken> {
    let adc = ApplicationDefaultCredentials::from_file().await?;

    let client = reqwest::Client::new();
    let res = client
        .post(GOOGLE_OAUTH2_TOKEN_ENDPOINT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&UserTokenRequest {
            grant_type: DEFAULT_USER_GRANT,
            refresh_token: &adc.refresh_token,
            client_id: &adc.client_id,
            client_secret: &adc.client_secret,
        })
        .send()
        .await
        .map_err(Error::OAuthRequest)?;
    if !res.status().is_success() {
        return Err(Error::BadRequest(res.status()).into());
    }

    let token_response: TokenResponse = res.json().await.map_err(Error::OAuthRequest)?;
    Ok(AccessToken {
        value: token_response.access_token,
        expires: Some(Utc::now() + Duration::seconds(token_response.expires_in)),
    })
}
