use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::errors::{LNError, LNErrorKind};
use crate::tipping::TippingRequest;
use crate::types::Invoice;
use crate::types::Quote;

pub struct QuoteRequest<'a> {
    api_key: &'a str,
    invoice_id: &'a str,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<(&'a TippingRequest<'a>, &'a Invoice)> for QuoteRequest<'a> {
    fn from((tipping_request, invoice): (&'a TippingRequest, &'a Invoice)) -> Self {
        QuoteRequest {
            api_key: tipping_request.api_key,
            invoice_id: invoice.invoice_id.as_str(),
            environment: tipping_request.environment,
            api_version: tipping_request.api_version,
        }
    }
}

impl<'a> QuoteRequest<'a> {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        return headers;
    }

    async fn request(&self) -> Result<Quote, LNErrorKind> {
        let quote_url = format!(
            "https://{}/{}/invoices/{}/quote",
            self.environment, self.api_version, self.invoice_id
        );

        let response = reqwest::Client::builder()
            .default_headers(self.get_headers())
            .build()?
            .post(&quote_url)
            .send()
            .await
            .map_err(|err| {
                LNErrorKind::StrikeError(LNError {
                    err: err.to_string(),
                })
            })?;

        match response.status() {
            reqwest::StatusCode::CREATED => {
                let quote: Quote = response.json().await.map_err(|err| {
                    LNErrorKind::JsonError(LNError {
                        err: err.to_string(),
                    })
                })?;
                Ok(quote)
            }
            _ => Err(LNErrorKind::StrikeError(LNError {
                err: format!("{}", response.status()),
            })),
        }
    }
}

pub async fn request_quote<'a, A>(quote_request: A) -> Result<Quote, LNErrorKind>
where
    A: Into<QuoteRequest<'a>>,
{
    let quote_request = quote_request.into();
    return quote_request.request().await;
}
