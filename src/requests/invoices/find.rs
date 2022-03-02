
use crate::requests::request::{Requestable};
use crate::types::{Invoice};
use crate::errors::{LNError};


pub struct FindInvoiceRequest<'a> {
    api_key: &'a str,
    invoice_id: &'a str,
    environment: &'a str,
    api_version: &'a str,
}


impl<'a> From<(&'a str, &'a str)> for FindInvoiceRequest<'a> {
    fn from((api_key,  invoice_id): (&'a str, &'a str)) -> Self {
        FindInvoiceRequest {
            api_key : api_key,
            invoice_id : invoice_id,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> Requestable for FindInvoiceRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/invoices/{}",
            self.environment, self.api_version, self.invoice_id
        )
    }
}

pub async fn find_invoice<'a, A>(find_invoice_request: A) -> Result<Invoice, LNError>
where
    A: Into<FindInvoiceRequest<'a>>,
{
    let find_invoice_request = find_invoice_request.into();
    find_invoice_request.get::<Invoice>().await
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::utils::get_api_key;
    use crate::requests::invoices::issue::test_invoice::test_issue_invoice;

   #[tokio::test]
   async fn test_find_invoice() {
        dotenv::dotenv().ok();
        let api_key = get_api_key();

        let invoice = test_issue_invoice().await;

        assert!(invoice.is_ok());

        let invoice = invoice.unwrap();

        let invoice_id = invoice.invoice_id;

        let find_invoice_request = find_invoice((&api_key[..], &invoice_id[..])).await;

        assert!(find_invoice_request.is_ok());
        let find_invoice_request = find_invoice_request.unwrap();
        assert_eq!(find_invoice_request.invoice_id, invoice_id);
   }
}
