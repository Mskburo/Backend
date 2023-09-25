// Just an example this will not compile

use async_trait::async_trait;
use sqlx::{Pool, Postgres, Error};


#[async_trait]
trait SimpleDbById<T> {
    async fn get_by_id(id: i32, connection: &Pool<Postgres>) -> Result<Option<T>, Error>;
    async fn delete_by_id(id: i32, connection: &Pool<Postgres>) -> Result<(), Error>;
}

// async fn insert(&self, connection: &Pool<Postgres>) -> Result<T, Error>;
// async fn create(input: T, connection: &Pool<Postgres>) -> Result<T, Error>;
// async fn get_all(connection: &Pool<Postgres>) -> Result<Vec<T>, Error>;
// async fn update_by_id(id: i32, connection: &Pool<Postgres>) -> Result<T, Error>;
// async fn delete(&self, connection: &Pool<Postgres>) -> Result<(), Error>;
// async fn delete_by_id(id: i32, connection: &Pool<Postgres>) -> Result<(), Error>;
