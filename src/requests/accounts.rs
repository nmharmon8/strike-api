
use crate::errors::{LNError};
use crate::types::{Account};

use crate::requests::request::{Requestable};

pub struct AccountHandleRequest<'a> {
    pub api_key: &'a str,
    pub handle: &'a str,
    pub environment: &'a str,
    pub api_version: &'a str,
}

impl<'a> Requestable for AccountHandleRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/accounts/handle/{}/profile",
            self.environment, self.api_version, self.handle
        )
    }
}

impl<'a> From<(&'a str, &'a str)> for AccountHandleRequest<'a> {
    fn from((api_key, handle): (&'a str, &'a str)) -> Self {
        AccountHandleRequest {
            api_key,
            handle: handle,
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}

pub async fn get_account_by_handle<'a, A>(rates_request: A) -> Result<Account, LNError>
where
    A: Into<AccountHandleRequest<'a>>,
{
    let rates_request = rates_request.into();
    rates_request.get::<Account>().await
}


#[cfg(test)]
mod test {
    use tokio;
    use super::*;
    use crate::test::utils::{get_api_key};

    #[tokio::test]
    async fn test_get_account_by_handel() {
        let api_key= get_api_key();
        let account_handle_request = get_account_by_handle((&api_key[..], "magog")).await;
        println!("{:?}", account_handle_request);
        assert!(account_handle_request.is_ok());
    }
}
