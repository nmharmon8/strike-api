use crate::types::{Subscription, Event};
use crate::requests::request::{Requestable};
use crate::errors::{LNError};
use serde::{Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubscriptionRequest<'a> {
    api_key: &'a str,
    #[serde(skip_serializing)]
    subscription_id: &'a str,
    webhook_url: &'a str,
    webhook_version: &'a str,
    secret: &'a str,
    enabled: bool,
    event_types: Vec<Event>,
    #[serde(skip_serializing)]
    environment: &'a str,
    #[serde(skip_serializing)]
    api_version: &'a str,
}

impl<'a> Requestable for UpdateSubscriptionRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/subscriptions/{}",
            self.environment, self.api_version, self.subscription_id
        )
    }

    fn get_body(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl<'a> From<(&'a str, &'a Subscription)> for UpdateSubscriptionRequest<'a> {
    fn from((api_key, subscription) : (&'a str, &'a Subscription)) -> Self {
        UpdateSubscriptionRequest {
            api_key,
            webhook_url: &subscription.webhook_url[..],
            subscription_id: &subscription.id[..],
            webhook_version: "v1",
            secret: &subscription.secret[..],
            enabled: subscription.enabled,
            event_types: subscription.event_types.clone(),
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}


pub async fn update_subscription<'a, A>(subscription_request: A) -> Result<Subscription, LNError>
where
    A: Into<UpdateSubscriptionRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    subscription_request.patch::<Subscription>().await
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::requests::subscriptions::delete::test_delete::test_delete_subscription;
    use crate::requests::subscriptions::create::test_create::test_create_subscription;
    use crate::test::utils::{get_api_key};

   #[tokio::test]
   async fn test_update_subscriptions() {

        let api_key = get_api_key();

        //Create a subscription to delete
        let subscription = test_create_subscription().await;
        assert!(subscription.is_ok());

        let subscription = subscription.unwrap();

        //Update the subscription
        let subscription_request = update_subscription((&api_key[..], &subscription)).await;
        println!("{:?}", subscription_request);
        assert!(subscription_request.is_ok());

        let updated_subscription = subscription_request.unwrap();
        
        //Delete the subscription
        test_delete_subscription(&updated_subscription.id[..]).await;
   }
}
