use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}