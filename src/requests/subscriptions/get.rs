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
mod tests {
    use super::*;
    use dotenv;
    use std::env;

   #[tokio::test]
   async fn test_get_subscriptions() {
        dotenv::dotenv().ok();
        let api_key = &env::var("API_KEY").unwrap_or("".to_string())[..];
        let subscription_request = GetSubscriptionsRequest::from(api_key);
        let subscriptions = get_subscriptions(subscription_request).await;
        assert!(subscriptions.is_ok());
   }
}
