use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::excursion::Excursion;
use crate::schema::{customers_types, customers_type_costs};

#[derive(Queryable,AsChangeset, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = customers_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomersTypes{
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Queryable, Selectable,Identifiable, Associations, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(CustomersTypes))]
#[diesel(belongs_to(Excursion))]
#[diesel(table_name = customers_type_costs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomersTypeCosts {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub customers_types_id: i32,
    pub cost: f64,
    pub excursion_id: i32
}
