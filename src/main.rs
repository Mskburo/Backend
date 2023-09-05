pub mod controllers;
pub mod models;
pub mod schema;

use actix_cors::Cors;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use controllers::{
    customer_costs::{
        add_customer_cost, delete_customer_cost_by_id, get_customer_cost_by_excursion_id,
        update_customer_cost_by_id,
    },
    customer_types_controller::{
        add_customer_type, delete_customer_type_by_id, get_all_customer_type,
        get_customer_type_by_id, update_customer_type_by_id,
    },
    excursion_controller::{
        add_excursion, delete_excursion_by_id, get_all_excursions, get_excursion_by_id,
        update_excursion_by_id,
    },
};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenv::dotenv;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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
        App::new().wrap(cors).service(
            web::scope("api/v1")
                .app_data(web::Data::new(
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
                //ADMIN
                .service(
                    web::scope("/admin/excursions")
                        .service(add_excursion)
                        .service(delete_excursion_by_id)
                        .service(update_excursion_by_id)
                        .service(
                            web::scope("/costs")
                                .service(add_customer_cost)
                                .service(delete_customer_cost_by_id)
                                .service(update_customer_cost_by_id)
                                .service(
                                    web::scope("/types")
                                        .service(add_customer_type)
                                        .service(delete_customer_type_by_id)
                                        .service(update_customer_type_by_id),
                                ),
                        ),
                )
                //CLIENT
                .service(
                    web::scope("/excursions")
                        .service(get_all_excursions)
                        .service(get_excursion_by_id)
                        .service(
                            web::scope("/costs")
                                .service(get_customer_cost_by_excursion_id)
                                .service(
                                    web::scope("/types")
                                        .service(get_all_customer_type)
                                        .service(get_customer_type_by_id),
                                ),
                        ),
                ), 
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
