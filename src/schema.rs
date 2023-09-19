// @generated automatically by Diesel CLI.

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

diesel::table! {
    orders (id) {
        id -> Int4,
        excursion_id -> Int4,
        #[max_length = 50]
        date -> Varchar,
        #[max_length = 50]
        time -> Varchar,
        name -> Text,
        #[max_length = 18]
        tel -> Varchar,
        email -> Text,
        #[max_length = 15]
        payment_type -> Varchar,
        bill -> Text,
        created_at -> Nullable<Timestamptz>,
        is_paid -> Bool,
    }
}

diesel::table! {
    orders_to_tickets (id) {
        id -> Int4,
        order_id -> Int4,
        customer_type_id -> Int4,
        amount -> Int4,
    }
}

diesel::joinable!(customers_type_costs -> customers_types (customers_types_id));
diesel::joinable!(customers_type_costs -> excursions (excursion_id));
diesel::joinable!(excursions -> excursions_types (excursion_type_id));
diesel::joinable!(orders -> excursions (excursion_id));
diesel::joinable!(orders_to_tickets -> customers_type_costs (customer_type_id));
diesel::joinable!(orders_to_tickets -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers_type_costs,
    customers_types,
    excursions,
    excursions_types,
    orders,
    orders_to_tickets,
);
