use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::errors::{LNError, LNErrorKind};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: String,
    pub amount: Amount,
    pub state: String,
    pub created: String,
    pub description: String,
    pub issuer_id: String,
    pub receiver_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub currency: String,
    pub amount: String,
}

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

impl<'a> From<(&'a str, &'a str, f32, &'a str)> for InvoiceRequest<'a> {
    fn from((api_key, account_handle, amount, currency): (&'a str, &'a str, f32, &'a str)) -> Self {
        InvoiceRequest {
            api_key: api_key,
            account_handle: account_handle,
            invoice_request_data: InvoiceRequestData {
                description: "Tip",
                amount: Amount {
                    amount: amount.to_string(),
                    currency: String::from(currency),
                },
                correlation_id: "",
            },
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}

impl<'a> From<(&'a str, &'a str, f32, &'a str, &'a str)> for InvoiceRequest<'a> {
    fn from(
        (api_key, account_handle, amount, currency, description): (
            &'a str,
            &'a str,
            f32,
            &'a str,
            &'a str,
        ),
    ) -> Self {
        InvoiceRequest {
            api_key: api_key,
            account_handle: account_handle,
            invoice_request_data: InvoiceRequestData {
                description: description,
                amount: Amount {
                    amount: amount.to_string(),
                    currency: String::from(currency),
                },
                correlation_id: "",
            },
            environment: "api.strike.me",
            api_version: "v1",
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

        reqwest::Client::builder()
            .default_headers(self.get_headers())
            .build()?
            .post(&invoice_url)
            .send()
            .await
            .map_err(|err| {
                LNErrorKind::StrikeError(LNError {
                    err: err.to_string(),
                })
            })?
            .json::<Invoice>()
            .await
            .map_err(|err| {
                LNErrorKind::JsonError(LNError {
                    err: err.to_string(),
                })
            })
    }
}

pub async fn issue_invoice<'a, A>(invoice_request: A) -> Result<Invoice, LNErrorKind>
where
    A: Into<InvoiceRequest<'a>>,
{
    let invoice_request = invoice_request.into();
    invoice_request.request().await
}
