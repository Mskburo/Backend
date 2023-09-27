use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub date: chrono::naive::NaiveDate,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub bill: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(skip_deserializing)]
    pub is_paid: Option<bool>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct InsertCost {
    pub customer_type_cost_id: i32,
    pub amount: i32,
}
#[derive(Deserialize, Serialize)]
pub struct InsertCart {
    pub cart_info: Cart,
    pub tickets: Vec<InsertCost>,
}
#[derive(Deserialize, Serialize, FromRow)]
pub struct HelperSum {
    pub sum: f64,
}
