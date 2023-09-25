use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::models::excursion::Excursion;
use tracing::error;

use crate::AppState;



//CREATE
#[put("")]
async fn add_excursion(app_state: web::Data<AppState>, json: web::Json<Excursion>) -> HttpResponse {
    match json.into_inner().insert(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//READ
#[get("")]
async fn get_all_excursions(app_state: web::Data<AppState>) -> HttpResponse {
    match Excursion::get_all(&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

#[get("/{excursion_id}")]
async fn get_excursion_by_id(
    app_state: web::Data<AppState>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();
    match Excursion::get_by_id(excursion_id, &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//Update
#[post("/{excursion_id}")]
async fn update_excursion_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<Excursion>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();
   match json.into_inner().update(excursion_id,&app_state.db).await {
        Ok(result) => HttpResponse::Accepted().body(format!("{} rows affected", result)),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}

//DELETE
#[delete("/{excursion_id}")]
async fn delete_excursion_by_id(
    app_state: web::Data<AppState>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();

     match Excursion::delete_by_id(excursion_id, &app_state.db).await {
        Ok(_) => HttpResponse::Accepted().body("deleted"),
        Err(e) => {error!("{}",e); return HttpResponse::BadRequest().body(format!("{}", e));}
    }
}
