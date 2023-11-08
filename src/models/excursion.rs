use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::costs::CustomersTypeCostsReturn;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Excursion {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub excursion_type_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub time: String,
    pub available: i32,
    pub photo: String,
    pub route: Option<String>,
    pub short_route: Option<String>,
    pub meeting_info: Option<String>,
    pub is_active: Option<bool>,
    pub times: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExcursionType {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExcursionWithCosts {
    pub excursion: ExcursionDetails,
    pub tickets: Vec<CustomersTypeCostsReturn>,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExcursionDetails {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub type_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub time: String,
    pub available: i32,
    pub photo: String,
    pub route: Option<String>,
    pub short_route: Option<String>,
    pub meeting_info: Option<String>,
    pub is_active: Option<bool>,
    pub times: Vec<String>,
    pub week_days:i32,
}

#[derive(Deserialize, Serialize)]
struct ExcursionJoin {
    pub excursion_info: Excursion,
    pub tikets: Vec<super::costs::CustomersTypeCosts>,
}

#[derive(Deserialize, Serialize)]
pub struct ExcursionQuery {
    pub excursion_id: i32,
    pub time: String,
    pub date: chrono::naive::NaiveDate,
}
