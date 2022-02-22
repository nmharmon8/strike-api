use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::errors::{LNError, LNErrorKind};
use crate::tipping::TippingRequest;
use crate::types::{Amount, Invoice};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InvoiceRequestData<'a> {
    description: &'a str,
    amount: Amount,
    #[serde(skip)]
    pub correlation_id: &'a str,
}

pub struct InvoiceRequest<'a> {
    api_key: &'a str,
    account_handle: &'a str,
    invoice_request_data: InvoiceRequestData<'a>,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<&'a TippingRequest<'a>> for InvoiceRequest<'a> {
    fn from(tipping_request: &'a TippingRequest) -> Self {
        InvoiceRequest {
            api_key: tipping_request.api_key,
            account_handle: tipping_request.account_handle,
            invoice_request_data: InvoiceRequestData {
                description: tipping_request.description,
                amount: Amount {
                    amount: tipping_request.amount.to_string(),
                    currency: String::from(tipping_request.currency),
                },
                correlation_id: tipping_request.correlation_id,
            },
            environment: tipping_request.environment,
            api_version: tipping_request.api_version,
        }
    }
}

impl<'a> InvoiceRequest<'a> {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers
    }

    async fn request(&self) -> Result<Invoice, LNErrorKind> {
        let invoice_url = format!(
            "https://{}/{}/invoices/handle/{}/",
            self.environment, self.api_version, self.account_handle
        );

        let response = reqwest::Client::builder()
            .default_headers(self.get_headers())
            .build()?
            .post(&invoice_url)
            .body(serde_json::to_string(&self.invoice_request_data).unwrap())
            .send()
            .await
            .map_err(|err| {
                LNErrorKind::StrikeError(LNError {
                    err: err.to_string(),
                })
            })?;

        match response.status() {
            reqwest::StatusCode::CREATED => {
                let invoice: Invoice = response.json().await.map_err(|err| {
                    LNErrorKind::JsonError(LNError {
                        err: err.to_string(),
                    })
                })?;
                Ok(invoice)
            }
            _ => Err(LNErrorKind::StrikeError(LNError {
                err: response.text().await.unwrap(),
            })),
        }
    }
}

pub async fn issue_invoice<'a, A>(invoice_request: A) -> Result<Invoice, LNErrorKind>
where
    A: Into<InvoiceRequest<'a>>,
{
    let invoice_request = invoice_request.into();
    invoice_request.request().await
}
