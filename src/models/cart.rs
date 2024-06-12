use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub date: chrono::naive::NaiveDate,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub bill: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(skip_deserializing)]
    pub is_paid: Option<bool>,
    pub promo_qr_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartWithTotalCost {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub date: chrono::naive::NaiveDate,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub bill: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(skip_deserializing)]
    pub is_paid: Option<bool>,
    pub excursion_name: Option<String>,
    pub excursion_id: Option<i32>,
    pub total_cost: f64,
    pub meeting_info: Option<String>,
    pub promo_qr_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartWithTotalCostReduced {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub date: chrono::naive::NaiveDate,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub bill: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(skip_deserializing)]
    pub is_paid: Option<bool>,
    pub excursion_name: Option<String>,
    pub total_cost: f64,
    pub promo_qr_id: Option<i32>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct InsertCost {
    pub customer_type_cost_id: i32,
    pub amount: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct JoinedCostInfo {
    pub amount: i32,
    pub cost: f64,
    pub name: String,
}
#[derive(Deserialize, Serialize)]
pub struct InsertCart {
    pub cart_info: Cart,
    pub tickets: Vec<InsertCost>,
}

#[derive(Deserialize, Serialize)]
pub struct ReturnCart {
    pub cart_info: CartWithTotalCost,
    pub tickets: Vec<JoinedCostInfo>,
}
#[derive(Deserialize, Serialize, FromRow)]
pub struct HelperSum {
    pub sum: f64,
}
