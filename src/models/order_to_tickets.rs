use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::orders_to_tickets;

#[derive(Queryable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = orders_to_tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrdersToTickets{
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub amount: i32,
    pub order_id: i32,
    pub customer_type_id: i32,
}