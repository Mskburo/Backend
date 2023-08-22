// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
