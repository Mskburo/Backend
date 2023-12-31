use actix_web::{web, HttpResponse, put, get, delete, post};
use tracing::error;

use crate::{models::qrs::Qrs, AppState}; 

#[put("")]
async fn add_qrs(
    app_state: web::Data<AppState>,
    json: web::Json<Qrs>,
) -> HttpResponse {
     let qr: Qrs = json.into_inner();
    match qr.save( &app_state.db).await {
        Ok(result) => HttpResponse::Accepted().json(result),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::BadRequest().body(format!("{}", e));
        }
    }
}


#[get("/{id}")]
async fn get_qrs_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    let id = path.0;
    match Qrs::find_by_id(id, &app_state.db).await {
        Ok(result) => {
            if let Some(qrs) = result {
                HttpResponse::Ok().json(qrs)
            } else {
                HttpResponse::NotFound().body("Qrs not found")
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}
#[get("")]
async fn get_all_qrs(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    match Qrs::get_all(&app_state.db).await {
        Ok(result) => {
            HttpResponse::Ok().json(result)
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}

#[post("/{id}")]
async fn update_qr(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    json: web::Json<Qrs>,
) -> impl actix_web::Responder {
    let id = path.0;
    let qr = json.into_inner();
    match qr.update(id, &app_state.db).await {
        Ok(result) => {
            if result.is_some()  {
                return HttpResponse::Ok().json(result);
            } else {
                HttpResponse::NotFound().body("Qrs not found")
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}

#[get("/promo/{id}")]
async fn increment_qrs_count_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> impl actix_web::Responder {
    let id = path.0;
    match Qrs::update_count_by_id(id, &app_state.db).await {
        Ok(result) => {
            if result.is_some()  {
                return HttpResponse::PermanentRedirect().append_header(("location", "http://mskburo.ru")).finish();
            } else {
                HttpResponse::NotFound().body("Qrs not found")
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}

#[delete("/{id}")]
async fn delete_qrs_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    let id = path.0;
    match Qrs::delete_by_id(id, &app_state.db).await {
        Ok(result) => {
            if let Some(qrs) = result {
                HttpResponse::Accepted().json(qrs)
            } else {
                HttpResponse::NotFound().body("Qrs not found")
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}
