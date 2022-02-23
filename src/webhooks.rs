use crate::types::{Event, Subscription};
use crate::errors::{LNError, ResponseError};
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize};
use serde_json;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubscriptionRequest<'a> {
    #[serde(skip_serializing)]
    api_key: &'a str,
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

impl<'a> From<(&'a str, &'a str, &'a str, Vec<Event>)> for SubscriptionRequest<'a> {
    fn from((api_key, web_hookurl, secret, event_types) : (&'a str, &'a str, &'a str, Vec<Event>)) -> Self {
        SubscriptionRequest {
            api_key,
            webhook_url: web_hookurl,
            webhook_version: "v1",
            secret,
            enabled: true,
            event_types,
            environment: "api.strike.me",
            api_version: "v1",
        }
    }
}

impl<'a> SubscriptionRequest<'a> {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers
    }

    async fn request(&self) -> Result<Subscription, LNError> {
        let subscription_url = format!(
            "https://{}/{}/subscriptions/",
            self.environment, self.api_version
        );
        let body = serde_json::to_string(&self).unwrap();

        let response = reqwest::Client::builder()
            .default_headers(self.get_headers())
            .build()?
            .post(&subscription_url)
            .body(body)
            .send()
            .await
            .map_err(|e| LNError::HTTPError(e.to_string()))?;

        //There must be a better way to do this
        match response.status() {
            reqwest::StatusCode::CREATED => {
                response.json().await.map_err(|err| {
                    LNError::JsonError(err.to_string())
                })
            },
            _ => {
                Err(LNError::HTTPResponseError(ResponseError {
                    status: response.status().as_u16(),
                    err: response.text().await.unwrap_or("".to_string()),
                    })
                )
            }
        }
    }
}

pub async fn subscribe<'a, A>(subscription_request: A) -> Result<Subscription, LNError>
where
    A: Into<SubscriptionRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    subscription_request.request().await
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_subscription_request() {
        let subscription_request = SubscriptionRequest {
            api_key: "api_key",
            webhook_url: "webhook_url",
            webhook_version: "webhook_version",
            secret: "secret",
            enabled: true,
            event_types: vec![Event::InvoiceCreated],
            environment: "environment",
            api_version: "api_version",
        };
        let expected = r#"{"webhookUrl":"webhook_url","webhookVersion":"webhook_version","secret":"secret","enabled":true,"eventTypes":["invoice.created"]}"#;
        let actual = serde_json::to_string(&subscription_request).unwrap();
        assert_eq!(expected, actual);
    }

    use dotenv;
    use std::env;

    #[tokio::test]
    async fn subscription_request() {
        dotenv::dotenv().ok();

        let api_key = &env::var("API_KEY").unwrap_or("".to_string())[..];
        let web_hookurl = "https://cb37-98-43-151-233.ngrok.io/strike/webhooks/invoice_created";
        let secret = "dfsdfsdfsdfsdfewf7sd5fs6df67";
        let event_types = vec![Event::InvoiceCreated];

        let subscription = subscribe((api_key, web_hookurl, secret, event_types)).await;
        println!("{:?}", subscription);
        assert!(subscription.is_ok());
    }
}
