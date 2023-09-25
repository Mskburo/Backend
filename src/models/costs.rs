use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomersTypes {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomersTypeCosts {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub customers_types_id: i32,
    pub cost: f64,
    pub excursion_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomersTypeCostsReturn {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub customers_type_name: String,
    pub cost: f64,
    pub excursion_id: i32,
}
