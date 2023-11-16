use crate::models::costs::CustomersTypes;
use actix_web::{delete, get, post, put, web, HttpResponse};
use tracing::error;

use crate::AppState;

//CREATE
#[put("")]
async fn add_customer_type(
    app_state: web::Data<AppState>,
    json: web::Json<CustomersTypes>,
) -> HttpResponse {
    match json.into_inner().insert(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//READ
#[get("/")]
async fn get_all_customer_type(app_state: web::Data<AppState>) -> HttpResponse {
    match CustomersTypes::get_all(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

#[get("/{customer_type_id}")]
async fn get_customer_type_by_id(
    app_state: web::Data<AppState>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();
    match CustomersTypes::get_by_id(customer_type_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//Update
#[post("/{customer_type_id}")]
async fn update_customer_type_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<CustomersTypes>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();

   match json.into_inner().update(customer_type_id,&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().body(format!("{} rows affected", result)),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//DELETE
#[delete("/{customer_type_id}")]
async fn delete_customer_type_by_id(
    app_state: web::Data<AppState>,
    customer_type_id: web::Path<i32>,
) -> HttpResponse {
    let customer_type_id = customer_type_id.into_inner();

     match CustomersTypes::delete_by_id(customer_type_id, &app_state.db).await {
        Ok(_) => HttpResponse::Accepted().body("deleted"),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}
