use crate::types::{Event, Subscription};
use crate::errors::{LNError};
use serde::{Serialize};
use serde_json;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::requests::request::{Requestable};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionRequest<'a> {
    #[serde(skip_serializing)]
    api_key: &'a str,
    webhook_url: &'a str,
    webhook_version: &'a str,
    secret: String,
    enabled: bool,
    event_types: Vec<Event>,
    #[serde(skip_serializing)]
    environment: &'a str,
    #[serde(skip_serializing)]
    api_version: &'a str,
}

impl<'a> From<(&'a str, &'a str, Vec<Event>)> for CreateSubscriptionRequest<'a> {
    fn from((api_key, web_hookurl, event_types) : (&'a str, &'a str, Vec<Event>)) -> Self {

        // Generate a random secret for the environment
        //seeded and reseeded via EntropyRng https://docs.rs/rand/0.6.5/rand/rngs/struct.EntropyRng.html
        let secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        CreateSubscriptionRequest {
            api_key,
            webhook_url: web_hookurl,
            webhook_version: "v1",
            secret: secret,
            enabled: true,
            event_types,
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}

impl<'a> Requestable for CreateSubscriptionRequest<'a> {

    fn get_api_key(&self) -> &str {
        self.api_key
    }

    fn get_url(&self) -> String {
        format!(
            "https://{}/{}/subscriptions/",
            self.environment, self.api_version
        )
    }

    fn get_body(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }


}

pub async fn subscribe<'a, A>(subscription_request: A) -> Result<Subscription, LNError>
where
    A: Into<CreateSubscriptionRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    
    match subscription_request.post::<Subscription>().await {
        //Add the secret to the subscription
        Ok(subscription) => {
            Ok({Subscription {
                id: subscription.id,
                webhook_url: subscription.webhook_url,
                webhook_version: subscription.webhook_version,
                enabled: subscription.enabled,
                created: subscription.created,
                secret: subscription_request.secret,
                event_types: subscription_request.event_types,
            }})
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
pub mod test_create {
    use super::*;
    use dotenv;
    use std::env;

    pub async fn test_create_subscription() -> Result<Subscription, LNError> {
        dotenv::dotenv().ok();
        let api_key = &env::var("API_KEY").unwrap_or("".to_string())[..];
        let web_hookurl = "https://cb37-98-43-151-233.ngrok.io/strike/webhooks/invoice_created";
        let event_types = vec![Event::InvoiceCreated];
        subscribe((api_key, web_hookurl, event_types)).await
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::requests::subscriptions::create::test_create;
    use crate::requests::subscriptions::delete::test_delete::test_delete_subscription;

    #[test]
    fn test_serialize_subscription_request() {
        let subscription_request = CreateSubscriptionRequest {
            api_key: "api_key",
            webhook_url: "webhook_url",
            webhook_version: "webhook_version",
            secret: "secret".to_string(),
            enabled: true,
            event_types: vec![Event::InvoiceCreated],
            environment: "environment",
            api_version: "api_version",
        };
        let expected = r#"{"webhookUrl":"webhook_url","webhookVersion":"webhook_version","secret":"secret","enabled":true,"eventTypes":["invoice.created"]}"#;
        let actual = serde_json::to_string(&subscription_request).unwrap();
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    pub async fn test_create_subscription() {
        let subscription = test_create::test_create_subscription().await;
        println!("{:?}", subscription);
        assert!(subscription.is_ok());
        test_delete_subscription( &subscription.unwrap().id[..]).await;
    }
}