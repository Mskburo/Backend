use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::cart_to_costs_types;

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize, Insertable,AsChangeset)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(CustomersTypeCosts))]
#[diesel(table_name = cart_to_costs_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CartToCostsTypes{
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub amount: i32,
    pub cart_id: i32,
    pub customer_type_cost_id: i32,
}