use sqlx::PgPool;

use crate::models::payments::Payment;

impl Payment {
    pub async fn insert(&self, connection: &PgPool) -> Result<Payment, sqlx::Error> {
        let result = sqlx::query_as!(
            Payment,
            "INSERT INTO payments (cart_id, payment_id)
            VALUES ($1, $2)
            ON CONFLICT (cart_id) DO UPDATE SET payment_id = $2
            RETURNING *;",
            self.cart_id,
            self.payment_id
        )
        .fetch_one(connection)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(
        id: i32,
        connection: &PgPool,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let result = sqlx::query_as!(
            Payment,
            "SELECT * FROM payments WHERE cart_id = $1;",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }
}
