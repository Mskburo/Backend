use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::photos;

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(Excursion))]
#[diesel(table_name = photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photos{
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub path: String,
    pub excursion_id: i32,
}