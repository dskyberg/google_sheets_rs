pub use auth::*;
pub use error::Error;

pub mod auth;
pub mod error;
pub mod source;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_access_token() {
        let access_token = fetch_access_token().await.expect("oops");
        dbg!(access_token);
    }
}
