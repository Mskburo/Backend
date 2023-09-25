use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::costs::{CustomersTypeCosts, CustomersTypeCostsReturn};

impl CustomersTypeCosts {
    pub async fn insert(&self, connection: &PgPool) -> Result<Option<CustomersTypeCosts>, Error> {
        let result = sqlx::query_as!(
            CustomersTypeCosts,
            "INSERT INTO customers_type_costs (excursion_id, customers_types_id, cost)
            VALUES ($1, $2, $3)
            RETURNING *;",
            self.excursion_id,
            self.customers_types_id,
            self.cost
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(
        id: i32,
        connection: &PgPool,
    ) -> Result<Option<CustomersTypeCosts>, Error> {
        let result = sqlx::query_as!(
            CustomersTypeCosts,
            "SELECT * FROM customers_type_costs WHERE id = $1;",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }
    pub async fn get_by_excursion_id(
        id: i32,
        connection: &PgPool,
    ) -> Result<Vec<CustomersTypeCostsReturn>, Error> {
        let result = sqlx::query_as::<_, CustomersTypeCostsReturn>(
            "SELECT ctc.id, ctc.excursion_id, ct.name AS customers_type_name, ctc.cost
                FROM customers_type_costs ctc
                JOIN customers_types ct ON ctc.customers_types_id = ct.id
                WHERE ctc.id = $1;",
        )
        .bind(id)
        .fetch_all(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_all(connection: &PgPool) -> Result<Vec<CustomersTypeCosts>, Error> {
        let result = sqlx::query_as!(CustomersTypeCosts, "SELECT * FROM customers_type_costs;")
            .fetch_all(connection)
            .await?;

        Ok(result)
    }

    pub async fn update(&self, id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE customers_type_costs
            SET excursion_id = $1, customers_types_id = $2, cost = $3
            WHERE id = $4;",
            self.excursion_id,
            self.customers_types_id,
            self.cost,
            id
        )
        .execute(connection)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    pub async fn update_by_id_value(cost: f64, id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE customers_type_costs
            SET  cost = $1
            WHERE id = $2;",
            cost,
            id,
        )
        .execute(connection)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    pub async fn delete_by_id(id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!("DELETE FROM customers_type_costs WHERE id = $1;", id)
            .execute(connection)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }
}
