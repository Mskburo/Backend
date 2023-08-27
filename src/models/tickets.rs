use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::tickets;

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(Excursion))]
#[diesel(table_name = tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tickets{
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub amount: Option<i32>,
    pub customers_type_costs_id: i32,
}