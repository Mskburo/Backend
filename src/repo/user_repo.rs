use actix_web::web::Data;

use crate::{models::user::User, AppState};
impl User {
    pub async fn insert(&self, state: Data<AppState>) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "
        INSERT INTO users (login, password)
        VALUES ($1, $2)
        RETURNING login, password
        ",
        )
        .bind(&self.login)
        .bind(&self.password)
        .fetch_one(&state.db)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_name(login: &str, state: Data<AppState>) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "
        SELECT login, password
        FROM users
        WHERE login = $1
        ",
        )
        .bind(login)
        .fetch_one(&state.db)
        .await?;
        Ok(user)
    }
    pub async fn user_is_unique(login: &str, state: &Data<AppState>) -> Result<bool, sqlx::Error> {
        let count = sqlx::query_scalar!("SELECT count(id) FROM users WHERE login = $1 ", login,)
            .fetch_one(&state.db)
            .await?
            .unwrap_or(0);
        Ok(count == 0)
    }
    pub async fn is_unique(&self, state: &Data<AppState>) -> Result<bool, sqlx::Error> {
        let count =
            sqlx::query_scalar!("SELECT count(id) FROM users WHERE login = $1", &self.login,)
                .fetch_one(&state.db)
                .await?
                .unwrap_or(0);
        Ok(count == 0)
    }

    pub async fn get_user_by_id(user_id: i32, state: &Data<AppState>) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "
        SELECT login, password
        FROM users
        WHERE id = $1
        ",
        )
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;
        Ok(user)
    }
}
