use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

use crate::Error;

const USER_CREDENTIAL_FILE: &str = "application_default_credentials.json";
const GCLOUD_PATH_PART: &str = "gcloud";
const CONFIG_PATH_PART: &str = ".config";
const WINDOWS_APPDATA_ENV: &str = "APPDATA";
const UNIX_HOME_ENV: &str = "HOME";

#[derive(Debug, Deserialize)]
pub(super) struct ApplicationDefaultCredentials {
    #[allow(dead_code)]
    pub account: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[allow(dead_code)]
    pub universe_domain: String,
}

impl ApplicationDefaultCredentials {
    pub async fn from_file() -> Result<ApplicationDefaultCredentials> {
        let buf = well_known_file()?;
        let slice = tokio::fs::read(buf).await?;
        let user: ApplicationDefaultCredentials = serde_json::from_slice(&slice)?;
        Ok(user)
    }
}

/// Returns the path to a gcloud user credential.
pub(super) fn well_known_file() -> Result<PathBuf> {
    let mut path = PathBuf::new();
    if cfg!(windows) {
        let appdata = std::env::var(WINDOWS_APPDATA_ENV).map_err(Error::MissingEnv)?;
        path.push(appdata);
        path.push(GCLOUD_PATH_PART);
    } else {
        let home = std::env::var(UNIX_HOME_ENV).map_err(Error::MissingEnv)?;
        path.push(home);
        path.push(CONFIG_PATH_PART);
    }

    path.push(GCLOUD_PATH_PART);
    path.push(USER_CREDENTIAL_FILE);
    Ok(path)
}
