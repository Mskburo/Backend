use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting_for_capture")]
    WaitingForCapture,
    #[serde(rename = "type")]
    Succeeded,
    #[serde(rename = "succeeded")]
    Canceled,
}

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub event: String,
    pub object: Object,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    pub status: WebhookEventType,
    pub amount: Amount,
    pub description: String,
    pub recipient: Recipient,
    pub payment_method: PaymentMethod,
    pub created_at: String,
    pub expires_at: String,
    pub test: bool,
    pub paid: bool,
    pub refundable: bool,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {}

#[derive(Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub payment_method_type: String,
    pub id: String,
    pub saved: bool,
    pub title: String,
    pub account_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct Recipient {
    pub account_id: String,
    pub gateway_id: String,
}
