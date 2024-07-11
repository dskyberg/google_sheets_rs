use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to lookup env")]
    MissingEnv(#[from] std::env::VarError),
    #[error("Unable to make request to oauth endpoint")]
    OAuthRequest(#[from] reqwest::Error),
    #[error("Bad request with status: {0}")]
    BadRequest(reqwest::StatusCode),
}
