use strike_api::tipping::{tipping_request};
extern crate qrcode_generator;
use qrcode_generator::{QrCodeEcc};

// Currently I only have a async version of tipping
#[tokio::main]
async fn main() {

    //This is your API Key, if you don't have one you can get one at https://developer.strike.me/
    //If you apply with an email linked to an existing Strike account, you will get a API key faster.
    let api_key = "<Your API KEY>";
    //This is the account handle of the account you want to tip
    //Can be your own account or another account
    let account_handle = "magog";
    //This is the amount you want to tip
    let amount = 1.0;
    //The currency the amount is specified in
    let currency = "USD";
    
    //Call the strike api to get a lightning invoice
    let tipping_quote = tipping_request(
        (api_key, account_handle, amount, currency)
    ).await;

    //Check if the request was successful
    match tipping_quote {
        Ok(quote) => {
            create_qrcode(quote.ln_invoice);
        },
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn create_qrcode(ln_invoice : String) {
    //Create a qrcode from the ln_invoice,
    match qrcode_generator::to_png_to_file(ln_invoice.clone(), QrCodeEcc::Low, 1024, "ln_qrcode.png") {
        Ok(_) => {
            println!("QR Code created successfully for invoice: {}", ln_invoice);
        },
        Err(error) => {
            println!("Error creating QR Code: {:?}", error);
        }
    }
}