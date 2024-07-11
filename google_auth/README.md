# Google Auth
Most of this was lifted directly from [Google Cloud Rust](https://github.com/googleapis/google-cloud-rust)

## Authentication

### Get client.json

In [Google Cloud Console](https://console.cloud.google.com), create a project, create an OAuth2 credential, and download the credentials file.  Call it what you want, but in the following step, it's called `client.json`.

In order to get an access token run the following command.  This will launch a browser based OAuth authentication and consent flow:

```bash
gcloud auth application-default login --client-id-file client.json --scopes 'https://www.googleapis.com/auth/spreadsheets'
```

If successful, the result should be a file named `~/.config/gcloud/application_default_credentials.json`, that containns your credentials, as well as a refresh token. This file will be read by [sheet_fun_bin] in order to receive an access token.   You can verify your authentication (and print an access token) with the following command, then you are ready.

```bash
gcloud auth application-default print-access-token
```
