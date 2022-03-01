
use crate::errors::{LNError};
use crate::types::{Rate};

use crate::requests::request::{Requestable};

pub struct RatesRequest<'a> {
    pub api_key: &'a str,
    pub environment: &'a str,
    pub api_version: &'a str,
}

impl<'a> Requestable for RatesRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/rates/ticker/",
            self.environment, self.api_version
        )
    }
}

impl<'a> From<&'a str> for RatesRequest<'a> {
    fn from(api_key: &'a str) -> Self {
        RatesRequest {
            api_key,
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}

pub async fn get_rates<'a, A>(rates_request: A) -> Result<Vec<Rate>, LNError>
where
    A: Into<RatesRequest<'a>>,
{
    let rates_request = rates_request.into();
    rates_request.get::<Vec<Rate>>().await
}


#[cfg(test)]
mod test {
    use tokio;
    use super::*;
    use crate::test::utils::{get_api_key};

    #[tokio::test]
    async fn test_get_rates() {
        let api_key= get_api_key();
        let rates = get_rates(&api_key[..]).await;
        assert!(rates.is_ok());
    }
}
