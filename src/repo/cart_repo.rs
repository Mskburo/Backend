use sqlx::postgres::PgPool;
use sqlx::{Connection, Error, Executor};

use crate::models::cart::Cart;
use crate::models::cart::{InsertCart, InsertCost};

impl InsertCart {
    pub async fn insert(&self, connection: &PgPool) -> Result<InsertCart, Error> {
        let cart = InsertCart::insert_or_return_existing(connection, &self.cart_info).await?;
        let mut tickets = vec![];
        for ticket in &self.tickets {
            let i = match InsertCart::insert_or_return_existing_ticket(connection, ticket, cart.id.unwrap())
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
        cart_id:i32,
    ) -> Result<InsertCost, sqlx::Error> {
        // Check if a cart with the same criteria already exists
        let existing_ticket: Option<InsertCost> = sqlx::query_as::<_, InsertCost>(
        "SELECT * FROM cart_to_costs_types WHERE customer_type_cost_id = $1 AND cart_id = $2 LIMIT 1;"
    )
    .bind(&ticket.customer_type_cost_id)
    .bind(cart_id)
    
    .fetch_optional(connection)
    .await?;
    

    if let Some(existing_ticket) = existing_ticket {
    let new_ticket: InsertCost = sqlx::query_as::<_, InsertCost>(
        "UPDATE  cart_to_costs_types SET amount = $1
            WHERE customer_type_cost_id = $2 AND cart_id = $3 RETURNING *;"
    )
    .bind(&ticket.amount)
    .bind(&ticket.customer_type_cost_id)
    .bind(cart_id)
    .fetch_one(connection)
    .await.map_err(|e| dbg!(e))?;


        return Ok(new_ticket);        
    }
    // If no existing cart is found, insert a new one
    let i =  sqlx::query_as::<_, InsertCost>(
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
    .bind(&cart_info.date)
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
            "INSERT INTO carts (date, time, name, tel, email, bill)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *;",
        )
        .bind(&cart_info.date)
        .bind(&cart_info.time)
        .bind(&cart_info.name)
        .bind(&cart_info.tel)
        .bind(&cart_info.email)
        .bind(&cart_info.bill)
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

    pub async fn get_all(connection: &PgPool) -> Result<Vec<InsertCart>, Error> {
        let mut result = vec![];
        let carts = sqlx::query_as::<_, Cart>("SELECT * FROM carts;")
            .fetch_all(connection)
            .await?;
        for cart in carts {
            let tickets = sqlx::query_as::<_, InsertCost>(
                "SELECT * FROM cart_to_costs_types  WHERE cart_id  = $1;",
            )
            .bind(cart.id)
            .fetch_all(connection)
            .await?;
            result.push(InsertCart {
                cart_info: cart.clone(),
                tickets,
            });
        }

        Ok(result)
    }
}
