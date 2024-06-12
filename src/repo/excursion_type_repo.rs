use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::excursion::ExcursionType;

impl ExcursionType {
    pub async fn insert(&self, connection: &PgPool) -> Result<Option<ExcursionType>, Error> {
        let result = sqlx::query_as!(
            ExcursionType,
            "INSERT INTO excursions_types (name)
            VALUES ($1)
            RETURNING *;",
            &self.name
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(id: i32, connection: &PgPool) -> Result<Option<ExcursionType>, Error> {
        let result = sqlx::query_as!(
            ExcursionType,
            "SELECT * FROM excursions_types WHERE id = $1;",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_all(connection: &PgPool) -> Result<Vec<ExcursionType>, Error> {
        let result = sqlx::query_as!(ExcursionType, "SELECT * FROM excursions_types;")
            .fetch_all(connection)
            .await?;

        Ok(result)
    }

    pub async fn update(&self, id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE excursions_types
            SET name = $1
            WHERE id = $2;",
            &self.name,
            id
        )
        .execute(connection)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    pub async fn delete_by_id(id: i32, connection: &PgPool) -> Result<u64, Error> {
        let rows_affected = sqlx::query!("DELETE FROM excursions_types WHERE id = $1;", id)
            .execute(connection)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }
}
