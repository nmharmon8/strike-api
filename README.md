# Strike Lightning API Interface in Rust (Unofficial)

<p align="center"><img  src="images/strike_lightning_rusty.png"></p>

Rust interface to Strike's excellent Lightning Network API. Easily add lightning tipping or payments to any application.

If you don't have an API key, apply for one [Strike](https://developer.strike.me/). In your application use an email with an existing strike account for quick approval.

## Tipping Example

Using Strikes API to tip someone, or even yourself, is very easy.

[Full Tipping Example Code](examples/rust_lightning_tipping_qrcode/)

Cargo.toml

```toml
[dependencies]
# For this example you must include the tipping feature
strike-api = { version = "0.0.1", features = ["tipping"] }
# Any qrcode generation library
qrcode-generator = {version = "4.1.2"}
# Any multi-threading library
tokio = {version = "1.17.0", features = ["full"]}
```

main.rs

```rust
use strike_api::tipping::{tipping_request};
extern crate qrcode_generator;
use qrcode_generator::{QrCodeEcc};

// Currently I only have a async version of tipping
#[tokio::main]
async fn main() {

    let api_key = "<Your API KEY>";
    //This is the account handle of the account you want to tip. Can be your own account or another account
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
```

This result in create a payable Lightning invoice, and a qr code saved to a png.

<p align="center"><img width=400 src="images/ln_qrcode.png"></p>
