use serde::{Deserialize, Serialize};
use std::fmt::{Debug};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub quote_id: String,
    pub description: String,
    pub ln_invoice: String,
    pub expiration: String,
    pub expiration_in_sec: i64,
    pub source_amount: SourceAmount,
    pub target_amount: TargetAmount,
    pub conversion_rate: ConversionRate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRate {
    pub amount: String,
    pub source_currency: String,
    pub target_currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoices {
    pub items: Vec<Invoice>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: String,
    pub amount: Amount,
    pub state: String,
    pub created: String,
    pub description: String,
    pub issuer_id: String,
    pub receiver_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub currency: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: String,
    pub webhook_url: String,
    pub webhook_version: String,
    pub enabled: bool,
    pub created: String,
    #[serde(skip_deserializing)]
    pub secret: String,
    pub event_types: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub amount: String,
    pub source_currency: String,
    pub target_currency: String,
}

fn none_string() -> String {
    String::from("None")
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub handle: String,
    #[serde(default = "none_string")]
    pub avatar_url: String,
    #[serde(default = "none_string")]
    pub description: String,
    pub can_receive: bool,
    pub currencies: Vec<Currency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub currency: String,
    pub is_default_currency: bool,
    pub is_available: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_subscription() {
        let subscription: Subscription = serde_json::from_str(
            r#"{
                "id": "4d0081e2-5355-411b-b0e4-ee5ff1b691d1",
                "webhookUrl": "https://kramerica_industries.com/webhook",
                "webhookVersion": "v1",
                "enabled": true,
                "created": "2022-02-23T18:29:18.773+00:00",
                "eventTypes": [
                  "invoice.created",
                  "invoice.updated"
                ]
              }"#,
        )
        .unwrap();

        assert_eq!(
            subscription,
            Subscription {
                id: "4d0081e2-5355-411b-b0e4-ee5ff1b691d1".to_string(),
                webhook_url: "https://kramerica_industries.com/webhook".to_string(),
                webhook_version: "v1".to_string(),
                enabled: true,
                secret: "".to_string(),
                created: "2022-02-23T18:29:18.773+00:00".to_string(),
                event_types: vec![Event::InvoiceCreated, Event::InvoiceUpdated],
            }
        );
    }
}
