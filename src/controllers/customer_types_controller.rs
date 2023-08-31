use crate::models::costs::CustomersTypes;
use crate::schema::customers_types::dsl::*;
use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use tracing::{error, warn};

use crate::AppState;

#[derive(serde::Deserialize)]
struct InsetCustomerType {
    name: String,
}

//CREATE
#[put("")]
async fn add_customer_type(
    app_state: web::Data<AppState>,
    query: web::Json<InsetCustomerType>,
) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::insert_into(customers_types)
                .values(crate::schema::customers_types::columns::name.eq(&query.into_inner().name))
                .execute(&mut conn)
                .await
            {
                Ok(inserted_rows) => {
                    HttpResponse::Ok().body(format!("{} customer type(s) added", inserted_rows))
                }
                Err(err) => {
                    warn!("Database error: {}", err);
                    let error_message = format!("Database error: {}", err);
                    HttpResponse::InternalServerError().body(error_message)
                }
            }
        }
        Err(err) => {
            error!("Database connection error: {}", err);
            let error_message = format!("Database connection error: {}", err);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}

//READ
#[get("")]
async fn get_all_customer_type(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => match customers_types.load::<CustomersTypes>(&mut conn).await {
            Ok(types) => HttpResponse::Ok().json(types),
            Err(err) => {
                warn!("Database error: {}", err);
                HttpResponse::InternalServerError().body(format!("Database error: {}", err))
            }
        },
        Err(err) => {
            error!("Database connection error: {}", err);
            HttpResponse::InternalServerError().body(format!("Database connection error: {}", err))
        }
    }
}

#[get("/{customer_type_id}")]
async fn get_customer_type_by_id(
    app_state: web::Data<AppState>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match customers_types
                .find(customer_type_id)
                .select(CustomersTypes::as_select())
                .first(&mut conn)
                .await
            {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(err) => {
                    warn!("Database error: {}", err);
                    HttpResponse::InternalServerError().body(format!("Database error: {}", err))
                }
            }
        }
        Err(err) => {
            error!("Database connection error: {}", err);
            HttpResponse::InternalServerError().body(format!("Database connection error: {}", err))
        }
    }
}

//Update
#[post("/{customer_type_id}")]
async fn update_customer_type_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<InsetCustomerType>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::update(
                customers_types
                    .filter(crate::schema::customers_types::columns::id.eq(customer_type_id)),
            )
            .set(crate::schema::customers_types::columns::name.eq(json.into_inner().name))
            .execute(&mut conn)
            .await
            {
                Ok(updated_rows) => {
                    if updated_rows > 0 {
                        HttpResponse::Ok().body(format!("{} excursion(s) updated", updated_rows))
                    } else {
                        HttpResponse::NotFound().body("Excursion not found")
                    }
                }
                Err(err) => {
                    warn!("Database error: {}", err);
                    HttpResponse::InternalServerError().body(format!("Database error: {}", err))
                }
            }
        }
        Err(err) => {
            error!("Database connection error: {}", err);
            HttpResponse::InternalServerError().body(format!("Database connection error: {}", err))
        }
    }
}

//DELETE
#[delete("/{customer_type_id}")]
async fn delete_customer_type_by_id(
    app_state: web::Data<AppState>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::delete(
                customers_types
                    .filter(crate::schema::customers_types::columns::id.eq(customer_type_id)),
            )
            .execute(&mut conn)
            .await
            {
                Ok(deleted_rows) => {
                    if deleted_rows > 0 {
                        HttpResponse::Ok()
                            .body(format!("{} customer types deleted", customer_type_id))
                    } else {
                        HttpResponse::NotFound().body("customer type not found")
                    }
                }
                Err(err) => {
                    warn!("Database error: {}", err);
                    HttpResponse::InternalServerError().body(format!("Database error: {}", err))
                }
            }
        }
        Err(err) => {
            error!("Database connection error: {}", err);
            HttpResponse::InternalServerError().body(format!("Database connection error: {}", err))
        }
    }
}
