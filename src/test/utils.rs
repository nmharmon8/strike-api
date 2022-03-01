use dotenv;
use std::env;

pub fn get_api_key() -> String {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").unwrap_or("".to_string());
    return api_key;
}