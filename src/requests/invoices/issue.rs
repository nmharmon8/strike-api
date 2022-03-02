
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


#[cfg(test)]
pub mod test_invoice {

    use super::*;
    use crate::test::utils::{get_api_key};

    pub async fn test_issue_invoice() -> Result<Invoice, LNError> {
        let api_key = get_api_key();
        let invoice_request = InvoiceRequest {
            api_key: &api_key[..],
            account_handle: "magog",
            invoice_request_data: InvoiceRequestData {
                description: "test_description",
                amount: Amount {
                    amount: "1.00".to_string(),
                    currency: "USD".to_string(),
                },
                correlation_id: "",
            },
            environment: "api.strike.me",
            api_version: "v1",
        };
        issue_invoice(invoice_request).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_issue_invoice() {
        let invoice = test_invoice::test_issue_invoice().await;
        println!("{:?}", invoice);
        assert!(invoice.is_ok());
    }
}