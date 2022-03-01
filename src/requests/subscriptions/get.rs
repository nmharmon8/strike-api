use crate::types::{Subscription};
use crate::errors::{LNError};

use crate::requests::request::{Requestable};

pub struct GetSubscriptionsRequest<'a> {
    api_key: &'a str,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<&'a str> for GetSubscriptionsRequest<'a> {
    fn from(api_key : &'a str) -> Self {
        GetSubscriptionsRequest {
            api_key : api_key,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> Requestable for GetSubscriptionsRequest<'a> {

        fn get_api_key(&self) -> &str {
            self.api_key
        }

        fn get_url(&self) -> String {
            format!(
                "https://{}/{}/subscriptions/",
                self.environment, self.api_version
            )
        }
}

pub async fn get_subscriptions<'a, A>(subscription_request: A) -> Result<Vec<Subscription>, LNError>
where
    A: Into<GetSubscriptionsRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    subscription_request.get::<Vec<Subscription>>().await
}

#[cfg(test)]
pub mod test_get {
    use super::*;
    use crate::test::utils::{get_api_key};

    pub async fn test_get_subscriptions() -> Result<Vec<Subscription>, LNError> {
        let api_key= get_api_key();
        let get_subscriptions = get_subscriptions(&api_key[..]).await;
        assert!(get_subscriptions.is_ok());
        get_subscriptions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[tokio::test]
   async fn test_get_subscriptions() {
        let subscriptions  = test_get::test_get_subscriptions().await;
        assert!(subscriptions.is_ok());
   }
}
