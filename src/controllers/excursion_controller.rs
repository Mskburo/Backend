use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        costs::CustomersTypeCosts,
        excursion::{ExcursionDetails, ExcursionType},
    },
    schema::excursions::dsl::*,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Object, RunQueryDsl};
use tracing::{error, warn};

use crate::{models::excursion::Excursion, AppState};

type _DbConnection = Object<
    diesel_async::pooled_connection::AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>,
>;

#[derive(Deserialize, Serialize)]
struct ExcursionJoin {
    pub excursion_info: Excursion,
    // pub excursion_type: ExcursionType,
    pub tikets: Vec<CustomersTypeCosts>,
}

//CREATE
#[put("")]
async fn add_excursion(app_state: web::Data<AppState>, json: web::Json<Excursion>) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::insert_into(excursions)
                .values(&json.into_inner())
                .returning(id)
                .get_result::<i32>(&mut conn)
                .await
            {
                Ok(returned_id) => {
                    HttpResponse::Ok().body(format!("excursion with id {} added", returned_id))
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
async fn get_all_excursions(app_state: web::Data<AppState>) -> HttpResponse {
    let mut result: Vec<ExcursionJoin> = vec![];

    let mut conn = match app_state.db.get().await {
        Ok(conn) => conn,
        Err(err) => {
            error!("Database connection error: {}", err);
            let error_message = format!("Database connection error: {}", err);
            return HttpResponse::InternalServerError().body(error_message);
        }
    };

    match excursions.load::<Excursion>(&mut conn).await {
        Ok(_excursions) => {
            for ex in _excursions {
                let tikets = match crate::schema::customers_type_costs::table
                    .filter(
                        crate::schema::customers_type_costs::columns::excursion_id
                            .eq(ex.id.unwrap_or(1)),
                    )
                    .select(CustomersTypeCosts::as_select())
                    .load(&mut conn)
                    .await
                {
                    Ok(result) => result,
                    Err(err) => {
                        warn!("Database error: {}", err);
                        return HttpResponse::InternalServerError()
                            .body(format!("Database error: {}", err));
                    }
                };

                result.push(ExcursionJoin {
                    excursion_info: ex,
                    tikets,
                });
            }
            return HttpResponse::Ok().json(result);
        }
        Err(err) => {
            warn!("Database error: {}", err);
            return HttpResponse::InternalServerError().body(format!("Database error: {}", err));
        }
    };
}

#[get("/{excursion_id}")]
async fn get_excursion_by_id(
    app_state: web::Data<AppState>,
    excursion_id: web::Path<i32>,
) -> HttpResponse {
    let excursion_id = excursion_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match excursions
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
            match diesel::update(excursions.filter(id.eq(excursion_id)))
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
            match diesel::delete(excursions.filter(id.eq(excursion_id)))
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
