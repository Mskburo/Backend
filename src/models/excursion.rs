pub(crate) use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{excursion,excursion_type};

#[derive(Queryable, Selectable ,Insertable, AsChangeset ,Identifiable, Associations, Debug, PartialEq , Deserialize, Serialize)]
#[diesel(belongs_to(ExcursionType))]
#[diesel(table_name = excursion)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Excursion {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub excursion_type_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub route: Option<String>,
    pub time: String,
    pub available: i32,
}


#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = excursion_type)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct ExcursionType {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
}
