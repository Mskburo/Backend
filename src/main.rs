pub mod models;
pub mod schema;
pub mod controllers;

use actix_web::{
    web::{self},
    App, HttpServer,
};
use controllers::excursion_controller::{add_excursion, get_all_excursions, get_excursion_by_id, delete_excursion_by_id, update_excursion_by_id};
use diesel_async::{pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use dotenv::dotenv;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use actix_cors::Cors;

#[derive(Clone)]
pub struct AppState {
    db: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    http_client: HttpPaymentClient,
}

#[derive(Clone)]
pub struct HttpPaymentClient {
    client: reqwest::Client,
    store_id: String,
    store_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .max_size(10)
        .build()
        .expect("error building pool");
    

    let store_id = std::env::var("YOOCASSA_STORE_ID").expect("YOOCASSA_STORE_ID must be set");
    let store_key = std::env::var("YOOCASSA_KEY").expect("YOOCASSA_KEY must be set");


    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        .service(
            web::scope("api/v1")
                .app_data(
                    web::Data::new(
                    AppState {
                        db: pool.clone(),
                        http_client: HttpPaymentClient {
                            client: reqwest::Client::new(),
                            store_id: store_id.as_str().to_string(),
                            store_key: store_key.as_str().to_string(),
                        },
                    }
                    .clone(),
                ))
                .service(add_excursion)
                .service(get_all_excursions)
                .service(get_excursion_by_id)
                .service(delete_excursion_by_id)
                .service(update_excursion_by_id),
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}