use crate::requests::subscriptions::subscription::SubscriptionsRequest;
use crate::requests::request::{Requestable};
use crate::types::Subscription;
use crate::errors::{LNError};


pub async fn find_subscription<'a, A>(subscription_request: A) -> Result<Subscription, LNError>
where
    A: Into<SubscriptionsRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    subscription_request.get::<Subscription>().await
}


#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    use crate::requests::subscriptions::delete::test_delete::test_delete_subscription;

    use crate::requests::subscriptions;

   #[tokio::test]
   async fn test_find_subscriptions() {
        dotenv::dotenv().ok();
        let api_key = &env::var("API_KEY").unwrap_or("".to_string())[..];

        //Create a subscription to delete
        let subscription = subscriptions::create::test_create::test_create_subscription().await;
        assert!(subscription.is_ok());

        let subscription = subscription.unwrap();

        //Find the subscription
        let subscription_found = find_subscription((api_key, &subscription.id[..])).await;
        assert!(subscription_found.is_ok());
        let subscription_found = subscription_found.unwrap();
        assert_eq!(subscription_found.id, subscription.id);
        
        //Delete the subscription
        test_delete_subscription(&subscription_found.id[..]).await;
   }
}
