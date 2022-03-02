use crate::requests::request::{Requestable};
use crate::types::{Invoice};
use crate::errors::{LNError};


pub struct CancelInvoiceRequest<'a> {
    api_key: &'a str,
    invoice_id: &'a str,
    environment: &'a str,
    api_version: &'a str,
}


impl<'a> From<(&'a str, &'a str)> for CancelInvoiceRequest<'a> {
    fn from((api_key,  invoice_id): (&'a str, &'a str)) -> Self {
        CancelInvoiceRequest {
            api_key : api_key,
            invoice_id : invoice_id,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> Requestable for CancelInvoiceRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        let url = format!(
            "https://{}/{}/invoices/{}/cancel",
            self.environment, self.api_version, self.invoice_id
        );
        println!("{}", url);
        url
    }
}


pub async fn cancel_invoice<'a, A>(cancel_invoice_request: A) -> Result<(), LNError>
where
    A: Into<CancelInvoiceRequest<'a>>,
{
    let cancel_invoice_request = cancel_invoice_request.into();
    cancel_invoice_request.patch().await
}



#[cfg(test)]
pub mod test_invoice {
    use super::*;
    use crate::test::utils::get_api_key;


    pub async fn test_cancel_invoice(invoice_id: &str) -> Result<(), LNError> {
        let api_key = get_api_key();
        cancel_invoice((&api_key[..], invoice_id)).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::invoices::issue::test_invoice::test_issue_invoice;

   #[tokio::test]
   async fn test_cancel_invoice() {

        let invoice = test_issue_invoice().await;

        println!("{:?}", invoice);

        assert!(invoice.is_ok());

        let invoice_id = invoice.unwrap().invoice_id;

        let result = test_invoice::test_cancel_invoice(&invoice_id[..]).await;

        println!("{:?}", result);

        assert!(result.is_ok());
    }
}