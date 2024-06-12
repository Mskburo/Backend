use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::cart::{
    Cart, CartWithTotalCost, CartWithTotalCostReduced, HelperSum, JoinedCostInfo, ReturnCart,
};
use crate::models::cart::{InsertCart, InsertCost};

impl InsertCart {
    pub async fn insert(&self, connection: &PgPool) -> Result<InsertCart, Error> {
        let cart = InsertCart::insert_or_return_existing(connection, &self.cart_info).await?;
        let mut tickets = vec![];
        for ticket in &self.tickets {
            let i = match InsertCart::insert_or_return_existing_ticket(
                connection,
                ticket,
                cart.id.unwrap(),
            )
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    return Err(e);
                }
            };

            tickets.push(i);
        }

        Ok(InsertCart {
            cart_info: cart,
            tickets,
        })
    }

    async fn insert_or_return_existing_ticket(
        connection: &PgPool,
        ticket: &InsertCost,
        cart_id: i32,
    ) -> Result<InsertCost, sqlx::Error> {
        // Check if a cart with the same criteria already exists
        let existing_ticket: Option<InsertCost> = sqlx::query_as::<_, InsertCost>(
        "SELECT * FROM cart_to_costs_types WHERE customer_type_cost_id = $1 AND cart_id = $2 LIMIT 1;"
    )
    .bind(ticket.customer_type_cost_id)
    .bind(cart_id)
    .fetch_optional(connection)
    .await?;

        if let Some(_existing_ticket) = existing_ticket {
            let new_ticket: InsertCost = sqlx::query_as::<_, InsertCost>(
                "UPDATE  cart_to_costs_types SET amount = $1
            WHERE customer_type_cost_id = $2 AND cart_id = $3 RETURNING *;",
            )
            .bind(ticket.amount)
            .bind(ticket.customer_type_cost_id)
            .bind(cart_id)
            .fetch_one(connection)
            .await
            .map_err(|e| dbg!(e))?;

            return Ok(new_ticket);
        }
        // If no existing cart is found, insert a new one
        let i = sqlx::query_as::<_, InsertCost>(
            "
        INSERT INTO cart_to_costs_types (customer_type_cost_id, amount, cart_id)
        VALUES ($1, $2, $3)
        RETURNING customer_type_cost_id, amount, cart_id;
        ",
        )
        .bind(ticket.customer_type_cost_id)
        .bind(ticket.amount)
        .bind(cart_id)
        .fetch_one(connection)
        .await?;

        Ok(i)
    }

    async fn insert_or_return_existing(
        connection: &PgPool,
        cart_info: &Cart, // Replace with the actual type of your cart info
    ) -> Result<Cart, sqlx::Error> {
        // Check if a cart with the same criteria already exists
        let existing_cart: Option<Cart> = sqlx::query_as::<_, Cart>(
        "SELECT * FROM carts WHERE date = $1 AND time = $2 AND name = $3 AND tel = $4 AND email = $5 AND bill = $6 LIMIT 1;"
    )
    .bind(cart_info.date)
    .bind(&cart_info.time)
    .bind(&cart_info.name)
    .bind(&cart_info.tel)
    .bind(&cart_info.email)
    .bind(&cart_info.bill)
    .fetch_optional(connection)
    .await?;

        // If an existing cart is found, return it
        if let Some(existing_cart) = existing_cart {
            return Ok(existing_cart);
        }

        // If no existing cart is found, insert a new one
        let cart: Cart = sqlx::query_as::<_, Cart>(
            "INSERT INTO carts (date, time, name, tel, email, bill, promo_qr_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *;",
        )
        .bind(cart_info.date)
        .bind(&cart_info.time)
        .bind(&cart_info.name)
        .bind(&cart_info.tel)
        .bind(&cart_info.email)
        .bind(&cart_info.bill)
        .bind(cart_info.promo_qr_id)
        .fetch_one(connection)
        .await?;

        Ok(cart)
    }

    pub async fn get_by_id(id: i32, connection: &PgPool) -> Result<InsertCart, Error> {
        let cart_info = sqlx::query_as::<_, Cart>("SELECT * FROM carts WHERE id = $1;")
            .bind(id)
            .fetch_one(connection)
            .await?;
        let tickets = sqlx::query_as::<_, InsertCost>(
            "SELECT * FROM cart_to_costs_types  WHERE cart_id  = $1;",
        )
        .bind(cart_info.id)
        .fetch_all(connection)
        .await?;

        Ok(InsertCart { cart_info, tickets })
    }

    pub async fn get_cost(&self, connection: &PgPool) -> Result<f64, Error> {
        let sum = sqlx::query_as::<_, HelperSum>(
            "
        SELECT SUM(cost.cost * cart_to_costs_types.amount)
        from cart_to_costs_types
        LEFT JOIN customers_type_costs cost on cart_to_costs_types.customer_type_cost_id = cost.id
        where 
        cart_to_costs_types.cart_id = $1;
        ",
        )
        .bind(self.cart_info.id)
        .fetch_one(connection)
        .await?;

        Ok(sum.sum)
    }

    pub async fn update_status_by_id(
        connection: &PgPool,
        cart_id: i32,
        new_status: bool,
    ) -> Result<Cart, Error> {
        let cart =
            sqlx::query_as::<_, Cart>("UPDATE carts SET is_paid = $1 WHERE id = $2 RETURNING *;")
                .bind(new_status)
                .bind(cart_id)
                .fetch_one(connection)
                .await?;
        Ok(cart)
    }
    pub async fn get_all_by_qr_id(
        connection: &PgPool,
        qr_id: i32,
        year: Option<u16>,
        month: Option<u8>,
        check_paid: bool,
    ) -> Result<Vec<CartWithTotalCostReduced>, Error> {
        let year_str = year
            .map(|f| format!("TO_DATE('{}', 'YYYY')", f))
            .unwrap_or("now()".to_string());

        let month_str = if let Some(y) = year {
            month
                .map(|f| format!("TO_DATE('{}-{}', 'YYYY-MM')", y, f))
                .unwrap_or("now()".to_string())
        } else {
            month
                .map(|f| {
                    format!(
                        "TO_DATE(EXTRACT(YEAR FROM CURRENT_DATE)::text || '-{}', 'YYYY-MM')",
                        f
                    )
                })
                .unwrap_or("now()".to_string())
        };

        let query = format!(
        "
        SELECT c.*, SUM(cost.cost * ctct.amount) as total_cost, excursions.name as excursion_name, payments.payment_id as bill
        FROM carts c
        LEFT JOIN cart_to_costs_types ctct ON c.id = ctct.cart_id
        LEFT JOIN customers_type_costs cost ON ctct.customer_type_cost_id = cost.id
        LEFT JOIN excursions ON cost.excursion_id = excursions.id
        LEFT JOIN payments ON c.id = payments.cart_id

        WHERE extract(YEAR FROM created_at) = extract(YEAR FROM {})
            and extract(MONTH FROM created_at) = extract(MONTH FROM {})
            AND promo_qr_id IS NOT NULL
            AND promo_qr_id = {}
            AND is_paid IS {}

        GROUP BY c.id, excursions.name, cost.excursion_id, payments.payment_id, meeting_info
        ORDER BY created_at DESC
        ",
            year_str,
            month_str,
            qr_id,
            check_paid
        );
        let carts = sqlx::query_as::<_, CartWithTotalCostReduced>(&query)
            .fetch_all(connection)
            .await?;

        Ok(carts)
    }

    pub async fn get_all(
        connection: &PgPool,
        is_sort_by_order_date: Option<bool>,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Result<Vec<ReturnCart>, Error> {
        let sort_column = if is_sort_by_order_date.unwrap_or(false) {
            "date"
        } else {
            "created_at"
        };
        let mut result: Vec<ReturnCart> = vec![];
        let query =         format!(
            "SELECT c.*, SUM(cost.cost * ctct.amount) as total_cost, excursions.name as excursion_name, cost.excursion_id, payments.payment_id as bill, excursions.meeting_info as meeting_info
            FROM carts c
            LEFT JOIN cart_to_costs_types ctct ON c.id = ctct.cart_id
            LEFT JOIN customers_type_costs cost ON ctct.customer_type_cost_id = cost.id
            LEFT JOIN excursions ON cost.excursion_id = excursions.id
            LEFT JOIN payments ON c.id = payments.cart_id
            {}
            GROUP BY c.id, excursions.name, cost.excursion_id, payments.payment_id, meeting_info
            ORDER BY {} DESC
            LIMIT 200",
            if date.is_some() { format!("WHERE {}::date = $1", sort_column) } else { "".to_owned() },
            sort_column
        );
        let carts = sqlx::query_as::<_, CartWithTotalCost>(&query)
            .bind(date) // Binding for ORDER BY clause
            .fetch_all(connection)
            .await?;
        // i dont like this
        for cart in carts {
            let tickets = sqlx::query_as::<_, JoinedCostInfo>(
                "SELECT types.amount, cost.cost, c_types.name
                        FROM cart_to_costs_types types
                            LEFT JOIN customers_type_costs cost ON types.customer_type_cost_id = cost.id
                            LEFT JOIN customers_types c_types ON cost.customers_types_id = c_types.id   
                    WHERE types.cart_id =$1;",
            )
            .bind(cart.id)
            .fetch_all(connection)
            .await?;
            result.push(ReturnCart {
                cart_info: cart.to_owned(),
                tickets,
            });
        }

        Ok(result)
    }
}
