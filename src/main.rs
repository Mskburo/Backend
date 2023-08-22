pub mod models;
pub mod schema;
use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use diesel_async::{pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use dotenv::dotenv;

pub struct AppState {
    db:  Arc<Pool<AsyncPgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .expect("error building connection");

    // checkout a connection from the pool
    let mut conn = pool.get().await.expect("error creating connection");

    HttpServer::new(|| App::new().service(web::scope("api/v1").app_data(AppState{db: Arc::new(pool.clone())})))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// let mut connection = AsyncPgConnection::establish(&std::env::var("DATABASE_URL")?).await?;
