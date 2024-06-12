use crate::models::costs::CustomersTypeCosts;
use actix_web::{delete, get, post, put, web, HttpResponse};

use tracing::error;

use crate::AppState;

//CREATE
#[put("")]
async fn add_customer_cost(
    app_state: web::Data<AppState>,
    json: web::Json<CustomersTypeCosts>,
) -> HttpResponse {
    match json.into_inner().insert(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            HttpResponse::BadRequest().body(format!("{}", e))
        }
    }
}

//READ by excursion id
#[get("/{excursion_id}")]
async fn get_customer_cost_by_excursion_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = path.into_inner();
    match CustomersTypeCosts::get_by_excursion_id(excursion_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            HttpResponse::BadRequest().body(format!("{}", e))
        }
    }
}

//Update
#[post("/{cost_id}")]
async fn update_customer_cost_by_id(
    app_state: web::Data<AppState>,
    new_cost: web::Json<f64>,
    path: web::Path<i32>,
) -> HttpResponse {
    let cost_id = path.into_inner();
    let new_cost = new_cost.into_inner();
    match CustomersTypeCosts::update_by_id_value(new_cost, cost_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            HttpResponse::BadRequest().body(format!("{}", e))
        }
    }
}

//DELETE
#[delete("/{customer_cost_id}")]
async fn delete_customer_cost_by_id(
    app_state: web::Data<AppState>,
    customer_cost_id: web::Path<i32>,
) -> HttpResponse {
    let customer_cost_id = customer_cost_id.into_inner();

    match CustomersTypeCosts::delete_by_id(customer_cost_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            HttpResponse::BadRequest().body(format!("{}", e))
        }
    }
}
