use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::{models::{
    cart::InsertCart,
    payments::{
        request::{Amount, Confirmation, PaymentRequest},
        response::PaymentResponse,
    },
}, controllers::payments_controller::create_payment};

use tracing::{debug, error, warn};

use crate::{models::cart::Cart, AppState};

//CREATE
#[put("")]
async fn add_cart(app_state: web::Data<AppState>, json: web::Json<InsertCart>) -> HttpResponse {
    match json.into_inner().insert(&app_state.db).await {
        Ok(result) => {
            return create_payment(app_state, result).await;
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::BadRequest().body(format!("{}", e));
        }
    }
}



//READ
#[get("")]
async fn get_all_carts(app_state: web::Data<AppState>) -> HttpResponse {
    match InsertCart::get_all(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::BadRequest().body(format!("{}", e));
        }
    }
}

// struct OrderResaponse{
//     cart: carts,
//     tickets: Vec<CustomersTypeCosts>
// }

#[get("/{cart_id}")]
async fn get_cart_by_id(app_state: web::Data<AppState>, input_id: web::Path<i32>) -> HttpResponse {
    let input_id = input_id.into_inner();

    match InsertCart::get_by_id(input_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::BadRequest().body(format!("{}", e));
        }
    }
}
