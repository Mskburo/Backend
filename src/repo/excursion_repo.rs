use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::models::excursion::{Excursion, ExcursionDetails, ExcursionQuery};

#[derive(Deserialize, Serialize,FromRow )]
struct QueryHelper{
    sold_tickets:i64
}

impl Excursion {
    pub async fn insert(
        &self,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<Excursion>, sqlx::Error> {
        let result = sqlx::query_as::<_, Excursion>(
            "INSERT INTO excursions (excursion_type_id, name, description, short_description, time, available, photo, route, short_route, meeting_info, is_active,times)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *;",
        )
        .bind(&self.excursion_type_id,)
        .bind(&self.name,)
        .bind(&self.description,)
        .bind(&self.short_description,)
        .bind(&self.time,)
        .bind(&self.available,)
        .bind(&self.photo,)
        .bind(&self.route,)
        .bind(&self.short_route,)
        .bind(&self.meeting_info,)
        .bind(&self.is_active ,)
        .bind(&self.times ,)
        .fetch_optional(connection)
        .await?;
        Ok(result)
    }
    pub async fn create(
        input: Excursion,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<Excursion>, sqlx::Error> {
        let result = sqlx::query_as::<_, Excursion>(
            "INSERT INTO excursions (excursion_type_id, name, description, short_description, time, available, photo, route, short_route, meeting_info, is_active, times)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *;",
        )
        .bind(input.excursion_type_id,)
        .bind(input.name,)
        .bind(input.description,)
        .bind(input.short_description,)
        .bind(input.time,)
        .bind(input.available,)
        .bind(input.photo,)
        .bind(input.route,)
        .bind(input.short_route,)
        .bind(input.meeting_info,)
        .bind(input.is_active ,)
        .bind(input.times ,)
        .fetch_optional(connection)
        .await?;
        Ok(result)
    }

    pub async fn get_by_id(
        id: i32,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<ExcursionDetails, sqlx::Error> {
        sqlx::query_as::<_, ExcursionDetails>(
            "
        SELECT *
        FROM excursion_details
        WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(connection)
        .await
    }

    pub async fn get_all(
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Vec<ExcursionDetails>, sqlx::Error> {
        sqlx::query_as::<_, ExcursionDetails>(
            "
        SELECT *
        FROM excursion_details WHERE is_active = true
        LIMIT 100;
        ",
        )
        .fetch_all(connection)
        .await
    }

    pub async fn update(
        &self,
        id: i32,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<u64, sqlx::Error> {
        let rows_affected = sqlx::query(
            "UPDATE excursions  SET excursion_type_id=$1, name=$2, description=$3, short_description=$4, time=$5, available=$6, photo=$7, route=$8, short_route=$9, meeting_info=$10, is_active=$11, times=$12 WHERE id = $13
        ",
            
        )
        .bind(&self.excursion_type_id,)
        .bind(&self.name,)
        .bind(&self.description,)
        .bind(&self.short_description,)
        .bind(&self.time,)
        .bind(&self.available,)
        .bind(&self.photo,)
        .bind(&self.route,)
        .bind(&self.short_route,)
        .bind(&self.meeting_info,)
        .bind(&self.is_active ,)
        .bind(&self.times ,)
        .bind(id,)
        .execute(connection)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

   
    pub async fn get_remaining(
        query: ExcursionQuery,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<i64, sqlx::Error> {
        let count: QueryHelper = sqlx::query_as::<_, QueryHelper>(
            "
            SELECT
                COALESCE(SUM(cart_to_costs_types.amount),0) as sold_tickets
            FROM
                carts 
            
            LEFT JOIN
                cart_to_costs_types ON carts.id = cart_to_costs_types.cart_id 

            LEFT JOIN
                customers_type_costs costs on cart_to_costs_types.customer_type_cost_id = costs.id AND costs.excursion_id = $1

            WHERE
                carts.date = $3 AND
                carts.time = $2 AND 
                cart_to_costs_types.customer_type_cost_id = costs.id;
        ",
            
        )
        .bind(query.excursion_id,)
        .bind(query.time,)
        .bind(query.date,)
        .fetch_one(connection)
        .await?;
        Ok(count.sold_tickets)
    }

    pub async fn delete(&self,connection: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            DELETE FROM excursions
            WHERE id = $1
            ",
        )
        .bind(&self.id)
        .execute(connection)
        .await?;
        Ok(())
    }

    pub async fn delete_by_id(
        id: i32,
        connection: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
         sqlx::query(
            "
            DELETE FROM excursions
            WHERE id = $1
            ",
        )
        .bind(id)
        .execute(connection)
        .await?;
        Ok(())
    }
}
