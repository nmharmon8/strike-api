use crate::types::{Invoices};
use crate::errors::{LNError};

use crate::requests::request::{Requestable};

pub struct GetInvoicesRequest<'a> {
    api_key: &'a str,
    filter: Option<&'a str>,
    order: Option<&'a str>,
    skip: Option<u32>,
    top: Option<u32>,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<&'a str> for GetInvoicesRequest<'a> {
    fn from(api_key : &'a str) -> Self {
        GetInvoicesRequest {
            api_key : api_key,
            filter : None,
            order : None,
            skip : None,
            top : None,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> From<(&'a str, Option<&'a str>, Option<&'a str>, Option<u32>, Option<u32>)> for GetInvoicesRequest<'a> {
    fn from((api_key, filter, order, skip, top): (&'a str, Option<&'a str>, Option<&'a str>, Option<u32>, Option<u32>)) -> Self {
        GetInvoicesRequest {
            api_key : api_key,
            filter : filter,
            order : order,
            skip : skip,
            top : top,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> Requestable for GetInvoicesRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        let mut url = format!(
            "https://{}/{}/invoices/",
            self.environment, self.api_version
        );

        let mut options = vec![];

        match self.filter {
            Some(filter) => {
                options.push(format!("filter={}", filter));

            },
            None => {},
        };

        match self.order {
            Some(order) => {
                options.push(format!("order={}", order));

            },
            None => {},
        };

        match self.skip {
            Some(skip) => {
                options.push(format!("skip={}", skip));

            },
            None => {},
        };

        match self.top {
            Some(top) => {
                options.push(format!("top={}", top));

            },
            None => {},
        };

        if options.len() > 0 {
            url.push_str("?");
            url.push_str(&options.join("&"));
        }
        url
    }
}

pub async fn get_invoices<'a, A>(invoice_request: A) -> Result<Invoices, LNError>
where
    A: Into<GetInvoicesRequest<'a>>,
{
    let invoice_request = invoice_request.into();
    invoice_request.get::<Invoices>().await
}

#[cfg(test)]
pub mod test_get {
    use super::*;
    use crate::test::utils::{get_api_key};

    pub async fn test_get_invoices() -> Result<Invoices, LNError> {
        let api_key = get_api_key();
        let invoices = get_invoices(&api_key[..]).await;
        assert!(invoices.is_ok());
        invoices
    }

    pub async fn test_get_invoices_with_query(filter: Option<&str>,  order : Option<&str>, skip : Option<u32>, top : Option<u32>) -> Result<Invoices, LNError> {
        let api_key = get_api_key();
        let invoices = get_invoices((&api_key[..], filter, order, skip, top)).await;
        assert!(invoices.is_ok());
        invoices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::invoices::issue;

   #[tokio::test]
   async fn test_get_invoices() {
        let invoices  = test_get::test_get_invoices().await;
        assert!(invoices.is_ok());
   }

    #[tokio::test]
    async fn test_get_invoices_with_query() {
        let invoice = issue::test_invoice::test_issue_invoice().await;
        assert!(invoice.is_ok());
        let invoice_id_filter = format!("invoiceId eq {}", invoice.unwrap().invoice_id);
        let invoices  = test_get::test_get_invoices_with_query(Some(&invoice_id_filter[..]), None, None, None).await;
        println!("{:?}", invoices);
        assert!(invoices.is_ok());
    }
}
