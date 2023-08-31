use crate::models::costs::CustomersTypeCosts;
use crate::schema::customers_type_costs::dsl::*;
use actix_web::{delete, get, post, web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use tracing::{error, warn};

use crate::AppState;


//CREATE
#[post("/excursions/costs")]
async fn add_customer_cost(
    app_state: web::Data<AppState>,
    json: web::Json<CustomersTypeCosts>,
) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::insert_into(customers_type_costs)
                .values(&json.into_inner())
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
#[get("/excursions/costs/{excursion_id}")]
async fn get_customer_cost_by_ex_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let excursion = path.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            
            match customers_type_costs
                .find(excursion)
                .select(CustomersTypeCosts::as_select())
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
#[post("/excursions/costs/{customer_cost_id}")]
async fn update_customer_cost_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<CustomersTypeCosts>,
    customer_cost_id: web::Path<i32>,
) -> HttpResponse {
    let customer_cost_id = customer_cost_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::update(
                customers_type_costs
                    .filter(crate::schema::customers_type_costs::columns::id.eq(customer_cost_id)),
            )
            .set(json.into_inner())
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
#[delete("/excursions/costs/{excursion_id}")]
async fn delete_customer_cost_by_id(
    app_state: web::Data<AppState>,
    customer_cost_id: web::Path<i32>,
) -> HttpResponse {
    let customer_cost_id = customer_cost_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::delete(
                customers_type_costs
                    .filter(crate::schema::customers_type_costs::columns::id.eq(customer_cost_id)),
            )
            .execute(&mut conn)
            .await
            {
                Ok(deleted_rows) => {
                    if deleted_rows > 0 {
                        HttpResponse::Ok().body(format!("{} excursion deleted", customer_cost_id))
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
