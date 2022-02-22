#[cfg(feature = "errors")]
pub mod errors;
#[cfg(feature = "tipping")]
mod invoice;
#[cfg(feature = "tipping")]
mod quote;
#[cfg(feature = "tipping")]
pub mod tipping;
#[cfg(feature = "types")]
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    #[tokio::test]
    async fn test_ln_tip() {
        dotenv::dotenv().ok();

        let quote: Result<types::Quote, errors::LNErrorKind> = tipping::tipping_request((
            &env::var("API_KEY").unwrap_or("".to_string())[..],
            &env::var("ACCOUNT_HANDLE").unwrap_or("".to_string())[..],
            1.0,
            "USD",
            "Description",
        ))
        .await;
        println!("{:?}", quote);
    }
}
