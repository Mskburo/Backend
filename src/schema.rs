// @generated automatically by Diesel CLI.

diesel::table! {
    cart_to_costs_types (id) {
        id -> Int4,
        cart_id -> Int4,
        customer_type_cost_id -> Int4,
        amount -> Int4,
    }
}

diesel::table! {
    carts (id) {
        id -> Int4,
        #[max_length = 50]
        date -> Varchar,
        #[max_length = 50]
        time -> Varchar,
        name -> Text,
        #[max_length = 20]
        tel -> Varchar,
        email -> Text,
        #[max_length = 15]
        payment_type -> Varchar,
        bill -> Text,
        created_at -> Nullable<Timestamptz>,
        is_paid -> Nullable<Bool>,
    }
}

diesel::table! {
    customers_type_costs (id) {
        id -> Int4,
        excursion_id -> Int4,
        customers_types_id -> Int4,
        cost -> Float8,
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
    excursions (id) {
        id -> Int4,
        excursion_type_id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        short_description -> Nullable<Text>,
        #[max_length = 50]
        time -> Varchar,
        available -> Int4,
        #[max_length = 50]
        photo -> Varchar,
        route -> Nullable<Text>,
        short_route -> Nullable<Text>,
        meeting_info -> Nullable<Text>,
        is_active -> Nullable<Bool>,
    }
}

diesel::table! {
    excursions_types (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::joinable!(cart_to_costs_types -> carts (cart_id));
diesel::joinable!(cart_to_costs_types -> customers_type_costs (customer_type_cost_id));
diesel::joinable!(customers_type_costs -> customers_types (customers_types_id));
diesel::joinable!(customers_type_costs -> excursions (excursion_id));
diesel::joinable!(excursions -> excursions_types (excursion_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart_to_costs_types,
    carts,
    customers_type_costs,
    customers_types,
    excursions,
    excursions_types,
);
