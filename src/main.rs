pub mod controllers;
pub mod models;
pub mod repo;
pub mod token;
pub mod validators;

use actix_cors::Cors;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use controllers::{
    auth::{basic_auth, create_user, generate_access},
    carts_controller::{add_cart, get_all_carts, get_cart_by_id},
    customer_costs::{
        add_customer_cost, delete_customer_cost_by_id, get_customer_cost_by_excursion_id,
        update_customer_cost_by_id,
    },
    customer_types_controller::{
        add_customer_type, delete_customer_type_by_id, get_all_customer_type,
        get_customer_type_by_id, update_customer_type_by_id,
    },
    excursion_controller::{
        add_excursion, delete_excursion_by_id, get_all_count_of_remaining_tickets,
        get_all_excursions, get_excursion_by_id, update_excursion_by_id,
    },
    payments_controller::*,
};

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use validators::{validator_acces, validator_refresh};

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let store_id = std::env::var("YOOCASSA_STORE_ID").expect("YOOCASSA_STORE_ID must be set");
    let store_key = std::env::var("YOOCASSA_KEY").expect("YOOCASSA_KEY must be set");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let bearer_middleware_refresh = HttpAuthentication::bearer(validator_refresh);
        let bearer_middleware_access = HttpAuthentication::bearer(validator_acces);
        let new_user_validator = HttpAuthentication::bearer(validators::new_user_validator);
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
                .service(
                    web::scope("/auth")
                        .service(basic_auth)
                        .service(
                            web::scope("/new")
                                .wrap(new_user_validator)
                                .service(create_user),
                        )
                        .service(
                            web::scope("/token")
                                .wrap(bearer_middleware_refresh)
                                .service(generate_access),
                        ),
                )
                //ADMIN
                .service(
                    web::scope("/admin")
                        .wrap(bearer_middleware_access)
                        .service(
                            web::scope("/carts")
                                .service(get_all_carts)
                                .service(get_cart_by_id),
                        )
                        .service(
                            web::scope("/excursions")
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
                        ),
                )
                //CLIENT
                .service(
                    web::scope("/excursions")
                        .service(get_all_excursions)
                        .service(get_excursion_by_id)
                        .service(get_all_count_of_remaining_tickets)
                        .service(
                            web::scope("/costs")
                                .service(get_customer_cost_by_excursion_id)
                                .service(
                                    web::scope("/types")
                                        .service(get_all_customer_type)
                                        .service(get_customer_type_by_id),
                                ),
                        ),
                )
                .service(
                    web::scope("/payments")
                        .service(capture_webhook_event)
                        .service(capture_payment_by_id),
                )
                .service(web::scope("/carts").service(add_cart)),
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
