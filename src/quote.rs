use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::errors::{LNError, LNErrorKind};

extern crate qrcode_generator;

use qrcode_generator::QrCodeEcc;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub quote_id: String,
    pub description: String,
    pub ln_invoice: String,
    pub expiration: String,
    pub expiration_in_sec: i64,
    pub source_amount: SourceAmount,
    pub target_amount: TargetAmount,
    pub conversion_rate: ConversionRate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRate {
    pub amount: String,
    pub source_currency: String,
    pub target_currency: String,
}

impl Quote {
    pub fn qrcode_as_png_file(&self, file_name: &str) -> Result<(), LNErrorKind> {
        qrcode_generator::to_png_to_file(self.ln_invoice.clone(), QrCodeEcc::Low, 1024, file_name)
            .map_err(|e| e.into())
    }

    pub fn qrcode_as_png(&self) -> Result<Vec<u8>, LNErrorKind> {
        qrcode_generator::to_png_to_vec(self.ln_invoice.clone(), QrCodeEcc::Low, 1024)
            .map_err(|e| e.into())
    }
}

pub struct QuoteRequest<'a> {
    api_key: &'a str,
    invoice_id: &'a str,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for QuoteRequest<'a> {
    fn from((api_key, invoice_id): (&'a str, &'a str)) -> Self {
        QuoteRequest {
            api_key: api_key,
            invoice_id: invoice_id,
            environment: "api.strike.me",
            api_version: "v1",
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
