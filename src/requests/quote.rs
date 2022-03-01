use crate::errors::{LNError};
use crate::types::Quote;
use crate::requests::request::{Requestable};

pub struct QuoteRequest<'a> {
    pub api_key: &'a str,
    pub invoice_id: &'a str,
    pub environment: &'a str,
    pub api_version: &'a str,
}

impl<'a> Requestable for QuoteRequest<'a>{

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/invoices/{}/quote",
            self.environment, self.api_version, self.invoice_id
        )
    }
}

pub async fn request_quote<'a, A>(quote_request: A) -> Result<Quote, LNError>
where
    A: Into<QuoteRequest<'a>>,
{
    let quote_request = quote_request.into();
    return quote_request.post::<Quote>().await;
}
