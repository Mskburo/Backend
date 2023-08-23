pub mod models;
pub mod schema;

use actix_web::{
    web::{self},
    App, HttpServer,
};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .max_size(10)
        .build()
        .expect("error building connection");

    // checkout a connection from the pool
    // let mut conn = pool.get().await.expect("error creating connection");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
        // .route("/{name}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}