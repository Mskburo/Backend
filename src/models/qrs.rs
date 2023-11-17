use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Qrs {
    pub id: Option<i32>,
    pub name: String,
    pub count: i32,
}