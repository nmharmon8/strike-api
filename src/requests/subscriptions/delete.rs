use crate::requests::subscriptions::subscription::SubscriptionsRequest;
use crate::requests::request::{Requestable};
use crate::errors::{LNError};


pub async fn delete_subscription<'a, A>(subscription_request: A) -> Result<(), LNError>
where
    A: Into<SubscriptionsRequest<'a>>,
{
    let subscription_request = subscription_request.into();
    subscription_request.delete().await
}


#[cfg(test)]
pub mod test_delete {
    use super::*;
    use crate::test::utils::{get_api_key};

    pub async fn test_delete_subscription(subscription_id: &str) {
        dotenv::dotenv().ok();
        let api_key= get_api_key();
        //Delete the subscription
        let delete_subscription = delete_subscription((&api_key[..], subscription_id)).await;
        assert!(delete_subscription.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::subscriptions;

   #[tokio::test]
   async fn test_delete_subscriptions() {
        //Create a subscription to delete
        let subscription = subscriptions::create::test_create::test_create_subscription().await;
        assert!(subscription.is_ok());
        let subscription = subscription.unwrap();
        test_delete::test_delete_subscription( &subscription.id[..]).await;
   }

//    use crate::requests::subscriptions::get::test_get::test_get_subscriptions;

//    #[tokio::test]
//    async fn test_delete_all_subscriptions() {
//         //Create a subscription to delete
//         let subscriptions = test_get_subscriptions().await.unwrap();
//         for subscription in subscriptions {
//             test_delete::test_delete_subscription( &subscription.id[..]).await;
//         }
//    }
}
