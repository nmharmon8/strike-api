use crate::requests::request::{Requestable};

pub struct SubscriptionsRequest<'a> {
    api_key: &'a str,
    subscription_id: &'a str,
    environment: &'a str,
    api_version: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for SubscriptionsRequest<'a> {
    fn from((api_key, subscription_id) : (&'a str, &'a str)) -> Self {
        SubscriptionsRequest {
            api_key : api_key,
            subscription_id: subscription_id,
            environment : "api.strike.me",
            api_version : "v1",
        }
    }
}

impl<'a> Requestable for SubscriptionsRequest<'a> {

        fn get_api_key(&self) -> &str {
            self.api_key
        }

        fn get_url(&self) -> String {
            format!(
                "https://{}/{}/subscriptions/{}",
                self.environment, self.api_version, self.subscription_id
            )
        }
}