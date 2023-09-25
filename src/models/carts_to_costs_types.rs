use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartToCostsTypes {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub amount: i32,
    pub cart_id: i32,
    pub customer_type_cost_id: i32,
}
