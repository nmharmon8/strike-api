pub mod errors;
pub mod invoice;
pub mod quote;
pub mod tipping;

use crate::errors::LNErrorKind;
use crate::invoice::issue_invoice;
use crate::tipping::TippingRequest;

pub async fn tipping_request<'a, A>(tipping_request: A) -> Result<quote::Quote, LNErrorKind>
where
    A: Into<TippingRequest<'a>>,
{
    let tipping_request = tipping_request.into();
    let invoice = issue_invoice(&tipping_request).await?;
    let quote = quote::request_quote((&tipping_request, &invoice)).await?;
    Ok(quote)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    #[tokio::test]
    async fn test_ln_tip() {
        dotenv::dotenv().ok();

        let quote = tipping_request((
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
