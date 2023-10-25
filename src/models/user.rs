use serde::{Serialize, Deserialize};
use sqlx::FromRow;



#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub login: String,
    pub password: String,
}
