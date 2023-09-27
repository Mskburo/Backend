use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Payment {
    pub cart_id: i32,
    pub payment_id: String,
}

pub mod request {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct PaymentRequest {
        pub amount: Amount,
        pub confirmation: Confirmation,
        pub description: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Amount {
        pub value: f64,
        pub currency: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Confirmation {
        #[serde(rename = "type")]
        pub confirmation_type: String,
        pub return_url: String,
    }
}

// RESPONSE
pub mod response {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PaymentResponse {
        pub id: String,
        pub status: String,
        pub amount: Amount,
        pub description: String,
        pub recipient: Recipient,
        pub created_at: String,
        pub confirmation: Confirmation,
        pub test: bool,
        pub paid: bool,
        pub refundable: bool,
        pub metadata: Metadata,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Amount {
        pub value: String,
        pub currency: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Confirmation {
        #[serde(rename = "type")]
        pub confirmation_type: String,
        pub confirmation_url: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Metadata {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Recipient {
        pub account_id: String,
        pub gateway_id: String,
    }
}

pub mod done_response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct PaymentDoneResponse {
        pub id: String,
        pub status: String,
        pub amount: Amount,
        pub income_amount: Amount,
        pub description: String,
        pub recipient: Recipient,
        pub payment_method: PaymentMethod,
        pub captured_at: String,
        pub created_at: String,
        pub test: bool,
        pub refunded_amount: Amount,
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
}
