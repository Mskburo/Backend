use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::schema::excursion::dsl::*;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Object, RunQueryDsl};
use tracing::{error, warn};

use crate::{models::excursion::Excursion, AppState};

type _DbConnection = Object<
    diesel_async::pooled_connection::AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>,
>;

//CREATE
#[put("")]
async fn add_excursion(app_state: web::Data<AppState>, json: web::Json<Excursion>) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::insert_into(excursion)
                .values(&json.into_inner())
                .execute(&mut conn)
                .await
            {
                Ok(inserted_rows) => {
                    HttpResponse::Ok().body(format!("{} excursion(s) added", inserted_rows))
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
#[get("/")]
async fn get_all_excursions(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => match excursion.load::<Excursion>(&mut conn).await {
            Ok(excursions) => HttpResponse::Ok().json(excursions),
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

#[get("/{excursion_id}")]
async fn get_excursion_by_id(
    app_state: web::Data<AppState>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match excursion
                .find(excursion_id)
                .select(Excursion::as_select())
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
#[post("/{excursion_id}")]
async fn update_excursion_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<Excursion>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::update(excursion.filter(id.eq(excursion_id)))
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
#[delete("/{excursion_id}")]
async fn delete_excursion_by_id(
    app_state: web::Data<AppState>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::delete(excursion.filter(id.eq(excursion_id)))
                .execute(&mut conn)
                .await
            {
                Ok(deleted_rows) => {
                    if deleted_rows > 0 {
                        HttpResponse::Ok().body(format!("{} excursion deleted", excursion_id))
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
