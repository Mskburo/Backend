// @generated automatically by Diesel CLI.

diesel::table! {
    customers_type_costs (id) {
        id -> Int4,
        customers_types_id -> Int4,
        cost -> Float8,
        excursion_id -> Int4,
    }
}

diesel::table! {
    customers_types (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    excursion (id) {
        id -> Int4,
        excursion_type_id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 50]
        time -> Varchar,
        available -> Int4,
    }
}

diesel::table! {
    excursion_type (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    photos (id) {
        id -> Int4,
        #[max_length = 255]
        path -> Varchar,
        excursion_id -> Int4,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        customers_type_costs_id -> Int4,
        amount -> Nullable<Int4>,
    }
}

diesel::joinable!(customers_type_costs -> customers_types (customers_types_id));
diesel::joinable!(customers_type_costs -> excursion (excursion_id));
diesel::joinable!(excursion -> excursion_type (excursion_type_id));
diesel::joinable!(photos -> excursion (excursion_id));
diesel::joinable!(tickets -> customers_type_costs (customers_type_costs_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers_type_costs,
    customers_types,
    excursion,
    excursion_type,
    photos,
    tickets,
);
