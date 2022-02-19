mod errors;
mod invoice;
mod quote;
use crate::errors::LNErrorKind;

use invoice::issue_invoice;

pub async fn ln_tip_png_to_vec(
    api_key: &str,
    account_handle: &str,
    amount: f32,
    currency: &str,
) -> Result<Vec<u8>, LNErrorKind> {
    let invoice = issue_invoice((api_key, account_handle, amount, currency)).await?;
    let quote = quote::request_quote((api_key, &invoice.invoice_id[..])).await?;
    quote.qrcode_as_png()
}

pub async fn ln_tip_png_to_file(
    file_name: &str,
    api_key: &str,
    account_handle: &str,
    amount: f32,
    currency: &str,
) -> Result<(), LNErrorKind> {
    let invoice = issue_invoice((api_key, account_handle, amount, currency)).await?;
    let quote = quote::request_quote((api_key, &invoice.invoice_id[..])).await?;
    quote.qrcode_as_png_file(file_name)
}

pub async fn ln_tip_invoice_id(
    file_name: &str,
    api_key: &str,
    account_handle: &str,
    amount: f32,
    currency: &str,
) -> Result<String, LNErrorKind> {
    let invoice = issue_invoice((api_key, account_handle, amount, currency)).await?;
    let quote = quote::request_quote((api_key, &invoice.invoice_id[..])).await?;
    Ok(quote.ln_invoice.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_ln_tip() {
        ln_tip_png_to_vec("<YourAPIKey>", "<YourAccountHandle>", 1.0, "USD").await;
    }
}
