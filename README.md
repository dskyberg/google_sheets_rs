# Spreadsheet Fun - Rust version

This app demonstrates how to intereact with Google Apps Script APIs.

## Setup

1. In GCP Dashboard, follow the steps to create a project.
2. Under APIs and Services -> Credentials, generate an OAuth client id
   1. Download the `client.json` file into the project root. If you use a different name, update the authentication command below appropriately.
3. Under APIs and Services -> Oauth Consent Screen, fill out, completely.
   1. Ensure the scopes you want are added
   2. Ensure test users are added

## Authenticating

See the section `Authentication` in [google_auth](google_auth/README.md)

## Run

This project uses Cargo workspaces and cargo-make.

You can use carg-make to run the app from either the `sheet_fun_bin` folder, or the parent `sheet_fun_rs` folder with

```bash
cargo run --package sheet_fun_bin
```
