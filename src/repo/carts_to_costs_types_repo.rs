use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::carts_to_costs_types::CartToCostsTypes;
use crate::models::costs::CustomersTypeCostsReturn;

impl CartToCostsTypes {
    pub async fn insert(&self, connection: &PgPool) -> Result<Option<CartToCostsTypes>, Error> {
        let result = sqlx::query_as!(
            CartToCostsTypes,
            "INSERT INTO cart_to_costs_types (cart_id, customer_type_cost_id, amount)
            VALUES ($1, $2, $3)
            RETURNING *;",
            self.cart_id,
            self.customer_type_cost_id,
            self.amount
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(
        id: i32,
        connection: &PgPool,
    ) -> Result<Option<CartToCostsTypes>, Error> {
        let result = sqlx::query_as!(
            CartToCostsTypes,
            "SELECT * FROM cart_to_costs_types WHERE id = $1;",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_all(connection: &PgPool) -> Result<Vec<CartToCostsTypes>, Error> {
        let result = sqlx::query_as!(CartToCostsTypes, "SELECT * FROM cart_to_costs_types;")
            .fetch_all(connection)
            .await?;

        Ok(result)
    }

    pub async fn update(&self, id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE cart_to_costs_types
            SET cart_id = $1, customer_type_cost_id = $2, amount = $3
            WHERE id = $4;",
            self.cart_id,
            self.customer_type_cost_id,
            self.amount,
            id
        )
        .execute(connection)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    pub async fn delete_by_id(id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!("DELETE FROM cart_to_costs_types WHERE id = $1;", id)
            .execute(connection)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    pub async fn get_all_types(
        id: i32,
        connection: &PgPool,
    ) -> Result<Vec<CustomersTypeCostsReturn>, Error> {
        let result = sqlx::query_as::<_, CustomersTypeCostsReturn>(
        "SELECT customers_type_costs.id,
                    name AS customers_type_name,
                    cost,
                    excursion_id
            FROM customers_type_costs
            LEFT JOIN customers_types ON customers_type_costs.customers_types_id = customers_types.id
            ;",
        ).bind(id)
        .fetch_all(connection)
        .await?;

        Ok(result)
    }
}
