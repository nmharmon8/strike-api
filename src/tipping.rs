use crate::errors;
use crate::requests::invoice;
use crate::requests::quote;
use crate::types;

impl<'a> From<(&'a TippingRequest<'a>, &'a types::Invoice)> for quote::QuoteRequest<'a> {
    fn from((tipping_request, invoice): (&'a TippingRequest, &'a types::Invoice)) -> Self {
        quote::QuoteRequest {
            api_key: tipping_request.api_key,
            invoice_id: invoice.invoice_id.as_str(),
            environment: tipping_request.environment,
            api_version: tipping_request.api_version,
        }
    }
}

impl<'a> From<&'a TippingRequest<'a>> for invoice::InvoiceRequest<'a> {
    fn from(tipping_request: &'a TippingRequest) -> Self {
        invoice::InvoiceRequest {
            api_key: tipping_request.api_key,
            account_handle: tipping_request.account_handle,
            invoice_request_data: invoice::InvoiceRequestData {
                description: tipping_request.description,
                amount: types::Amount {
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

pub struct TippingRequest<'a> {
    pub api_key: &'a str,
    pub account_handle: &'a str,
    pub amount: f32,
    pub currency: &'a str,
    pub description: &'a str,
    pub environment: &'a str,
    pub api_version: &'a str,
    pub correlation_id: &'a str,
}

impl<'a, T> From<(T, T, f32, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from((api_key, account_handle, amount, currency): (T, T, f32, T)) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: "Tip",
            environment: "api.strike.me",
            api_version: "v1",
            correlation_id: "",
        }
    }
}

impl<'a, T> From<(T, T, f32, T, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from((api_key, account_handle, amount, currency, description): (T, T, f32, T, T)) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: description.into(),
            environment: "api.strike.me",
            api_version: "v1",
            correlation_id: "",
        }
    }
}

impl<'a, T> From<(T, T, f32, T, T, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from(
        (api_key, account_handle, amount, currency, description, correlation_id): (
            T,
            T,
            f32,
            T,
            T,
            T,
        ),
    ) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: description.into(),
            environment: "api.strike.me",
            api_version: "v1",
            correlation_id: correlation_id.into(),
        }
    }
}

//Do you need to implement the From for the default constructor?
impl<'a, T> From<(T, T, f32, T, T, T, T, T)> for TippingRequest<'a>
where
    T: Into<&'a str>,
{
    fn from(
        (
            api_key,
            account_handle,
            amount,
            currency,
            description,
            environment,
            api_version,
            correlation_id,
        ): (T, T, f32, T, T, T, T, T),
    ) -> Self {
        TippingRequest {
            api_key: api_key.into(),
            account_handle: account_handle.into(),
            amount: amount,
            currency: currency.into(),
            description: description.into(),
            environment: environment.into(),
            api_version: api_version.into(),
            correlation_id: correlation_id.into(),
        }
    }
}

pub async fn tipping_request<'a, A>(tipping_request: A) -> Result<types::Quote, errors::LNError>
where
    A: Into<TippingRequest<'a>>,
{
    let tipping_request = tipping_request.into();
    let invoice = invoice::issue_invoice(&tipping_request).await?;
    let quote = quote::request_quote((&tipping_request, &invoice)).await?;
    Ok(quote)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    #[tokio::test]
    async fn test_ln_tip() {
        dotenv::dotenv().ok();

        let quote: Result<types::Quote, errors::LNError> = tipping_request((
            &env::var("API_KEY").unwrap_or("".to_string())[..],
            &env::var("ACCOUNT_HANDLE").unwrap_or("".to_string())[..],
            1.0,
            "USD",
            "Description",
        ))
        .await;
        println!("{:?}", quote);
    }
}
