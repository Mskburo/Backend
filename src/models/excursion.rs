pub(crate) use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{excursions, excursions_types};

#[derive(
    Queryable,
    AsChangeset,
    Insertable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Deserialize,
    Serialize,
)]
#[diesel(belongs_to(ExcursionType))]
#[diesel(table_name = excursions)]
#[diesel(primary_key(id))]
pub struct Excursion {
    #[diesel(deserialize_as = i32)]
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
}

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = excursions_types)]
#[diesel(primary_key(id))]
pub struct ExcursionType {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
}
