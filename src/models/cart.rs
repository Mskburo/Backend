use crate::schema::carts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize, Insertable, AsChangeset,
)]
#[diesel(belongs_to(Excursion))]
#[diesel(table_name = carts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cart {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub date: String,
    pub time: String,
    pub name: String,
    pub tel: String,
    pub email: String,
    pub bill: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_paid: Option<bool>,
}
