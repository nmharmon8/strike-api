
use serde::{Deserialize, Serialize};

use crate::errors::{LNError};
use crate::types::{Amount, Invoice};
use serde_json;

use crate::requests::request::{Requestable};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRequestData<'a> {
    pub  description: &'a str,
    pub amount: Amount,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub correlation_id: &'a str,
}

pub struct InvoiceRequest<'a> {
    pub api_key: &'a str,
    pub account_handle: &'a str,
    pub invoice_request_data: InvoiceRequestData<'a>,
    pub environment: &'a str,
    pub api_version: &'a str,
}

impl<'a> Requestable for InvoiceRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/invoices/handle/{}/",
            self.environment, self.api_version, self.account_handle
        )
    }

    fn get_body(&self) -> String {
        serde_json::to_string(&self.invoice_request_data).unwrap()
    }
}

pub async fn issue_invoice<'a, A>(invoice_request: A) -> Result<Invoice, LNError>
where
    A: Into<InvoiceRequest<'a>>,
{
    let invoice_request = invoice_request.into();
    invoice_request.post::<Invoice>().await
}
