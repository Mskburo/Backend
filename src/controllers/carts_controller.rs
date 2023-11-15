use actix_web::{get, put, web, HttpResponse};
use serde::Deserialize;

use crate::{models::cart::InsertCart, controllers::payments_controller::create_payment};

use tracing::error;

use crate::AppState;

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

#[derive(Deserialize)]
struct GetCartsQuery {
    #[serde(rename = "sortByExcursionDate")]
    sort_by_excursion_date: Option<bool>,
    date:  Option<chrono::naive::NaiveDate>
}

//READ
#[get("")]
async fn get_all_carts(app_state: web::Data<AppState>, info: web::Query<GetCartsQuery>) -> HttpResponse {
    match InsertCart::get_all(&app_state.db, info.sort_by_excursion_date, info.date).await {
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
