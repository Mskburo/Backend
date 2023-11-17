use sqlx::{Error, PgPool};

use crate::models::qrs::Qrs;

impl Qrs {
    pub async fn insert(qrs: &Qrs, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(
            Qrs,
            "INSERT INTO qrs (name, count)
            VALUES ($1, $2)
            RETURNING *;",
            qrs.name,
            qrs.count
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn save(&self, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(
            Qrs,
            "INSERT INTO qrs (id, name, count)
            VALUES ($1, $2, $3)
            RETURNING *;",
            &self.id.unwrap_or(1),
            &self.name,
            &self.count
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(id: i32, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(Qrs, "SELECT * FROM qrs WHERE id = $1;", id)
            .fetch_optional(connection)
            .await?;

        Ok(result)
    }
    pub async fn get_all(connection: &PgPool) -> Result<Vec<Qrs>, Error> {
        let result = sqlx::query_as!(Qrs, "SELECT * FROM qrs;")
            .fetch_all(connection)
            .await?;

        Ok(result)
    }

    pub async fn update(&self, id: i32, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(
            Qrs,
            "UPDATE qrs
            SET  name = $1, count = $2
            WHERE id = $3
            returning *",
            &self.name,
            &self.count,
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn update_count_by_id(id: i32, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(
            Qrs,
            "UPDATE qrs SET count = count + 1 WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn delete_by_id(id: i32, connection: &PgPool) -> Result<Option<Qrs>, Error> {
        let result = sqlx::query_as!(Qrs, "DELETE FROM qrs WHERE id = $1 RETURNING *;", id)
            .fetch_optional(connection)
            .await?;

        Ok(result)
    }
}
