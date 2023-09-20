use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    models::carts_to_costs_types::CartToCostsTypes,
    schema::{self, cart_to_costs_types::dsl::*, carts::dsl::*},
};

use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Object, RunQueryDsl};
use tracing::{error, warn};

use crate::{models::cart::Cart, AppState};

type _DbConnection = Object<
    diesel_async::pooled_connection::AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>,
>;

#[derive(Deserialize, Serialize)]
struct InsertCost {
    cost_id: i32,
    amount: i32,
}
#[derive(Deserialize, Serialize)]
struct InsertCart {
    cart_info: Cart,
    tickets: Vec<InsertCost>,
}

//CREATE
#[put("")]
async fn add_cart(app_state: web::Data<AppState>, json: web::Json<InsertCart>) -> HttpResponse {
    let input_cart = json.into_inner();
    let mut conn = match app_state.db.get().await {
        Ok(conn) => conn,
        Err(err) => {
            error!("Database connection error: {}", err);
            let error_message = format!("Database connection error: {}", err);
            return HttpResponse::InternalServerError().body(error_message);
        }
    };

    let cart = match diesel::insert_into(carts)
        .values(input_cart.cart_info)
        .returning(schema::carts::columns::id)
        .get_result::<i32>(&mut conn)
        .await
    {
        Ok(_id) => _id,
        Err(err) => {
            warn!("Database error: {}", err);
            let error_message = format!("Database error: {}", err);
            return HttpResponse::InternalServerError().body(error_message);
        }
    };

    let cart_to_costs: Vec<CartToCostsTypes> = input_cart.tickets
        .iter()
        .map(|insert_cost| CartToCostsTypes {
            id: None, // You can set this value as needed.
            amount: insert_cost.amount,
            cart_id: cart, // Set the cart_id to the desired value.
            customer_type_cost_id: insert_cost.cost_id,
        })
        .collect();

    match diesel::insert_into(cart_to_costs_types)
        .values(cart_to_costs)
        .returning(schema::cart_to_costs_types::columns::id)
        .get_result::<i32>(&mut conn)
        .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("cart with id {} added", cart)),
        Err(err) => {
            warn!("Database error: {}", err);
            let error_message = format!("Database error: {}", err);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}

//READ
#[get("")]
async fn get_all_carts(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.db.get().await {
        Ok(mut conn) => match carts.load::<Cart>(&mut conn).await {
            Ok(_carts) => HttpResponse::Ok().json(_carts),
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

// struct OrderResaponse{
//     cart: carts,
//     tickets: Vec<CustomersTypeCosts>
// }

#[get("/{cart_id}")]
async fn get_cart_by_id(app_state: web::Data<AppState>, input_id: web::Path<i32>) -> HttpResponse {
    let input_id = input_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match carts
                .find(input_id)
                .select(Cart::as_select())
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
#[post("/{cart_id}")]
async fn update_cart_by_id(
    app_state: web::Data<AppState>,
    json: web::Json<InsertCart>,
    input_id: web::Path<i32>,
) -> HttpResponse {
    let input_id = input_id.into_inner();
    let insert_cart = json.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::update(carts.filter(schema::carts::columns::id.eq(input_id)))
                .set(insert_cart.cart_info)
                .execute(&mut conn)
                .await
            {
                Ok(updated_rows) => {
                    if updated_rows > 0 {
                        HttpResponse::Ok().body(format!("{} rows updated", updated_rows))
                    } else {
                        HttpResponse::NotFound().body("cart not found")
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
#[delete("/{cart_id}")]
async fn delete_cart_by_id(
    app_state: web::Data<AppState>,
    input_id: web::Path<i32>,
) -> HttpResponse {
    let input_id = input_id.into_inner();

    match app_state.db.get().await {
        Ok(mut conn) => {
            match diesel::delete(carts.filter(schema::carts::columns::id.eq(input_id)))
                .execute(&mut conn)
                .await
            {
                Ok(deleted_rows) => {
                    if deleted_rows > 0 {
                        HttpResponse::Ok().body(format!("{} cart deleted", input_id))
                    } else {
                        HttpResponse::NotFound().body("cart not found")
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
