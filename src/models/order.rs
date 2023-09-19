use crate::schema::orders;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(Excursion))]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrdersToTickets {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub excursion_id: i32,
    pub date: String,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub payment_type: String,
    pub bill: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_paid: bool,
}
