use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse, Responder,
};

use serde_json::json;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{
    models::{
        cart::{Cart, InsertCart},
        payments::{
            done_response::PaymentDoneResponse,
            request::{Amount, Confirmation, PaymentRequest},
            response::PaymentResponse,
            Payment,
        },
        webhook_event::{Webhook, WebhookEventType},
    },
    AppState,
};

#[post("/capture")]
async fn capture_webhook_event(
    app_state: web::Data<AppState>,
    json: web::Json<Webhook>,
) -> HttpResponse {
    let event = json.into_inner();

    match event.object.status {
        WebhookEventType::Pending => return HttpResponse::Ok().body("ok"),
        WebhookEventType::WaitingForCapture => {
            match Payment::get_by_payment_id(event.object.id, &app_state.db).await {
                Ok(payment) => match payment {
                    Some(result) => return capture_payment(app_state, result.cart_id).await,
                    None => {
                        error!("no payments found.");
                        return HttpResponse::BadRequest().body(format!("error capturing payment"));
                    }
                },
                Err(e) => {
                    return HttpResponse::BadRequest()
                        .body(format!("error capturing payment \n {}", e));
                }
            };
        }
        WebhookEventType::Succeeded => return HttpResponse::Ok().body("ok"),
        WebhookEventType::Canceled => {
            match Payment::get_by_payment_id(event.object.id, &app_state.db).await {
                Ok(payment) => match payment {
                    Some(result) => {
                        match InsertCart::get_by_id(result.cart_id, &app_state.db).await {
                            Ok(cart) => {
                                if !cart.cart_info.is_paid.unwrap_or(false) {
                                    match update_cart_status(&app_state.db, result.cart_id).await {
                                        Some(_) => {
                                            return HttpResponse::Accepted()
                                                .body(result.payment_id);
                                        }
                                        None => {
                                            return HttpResponse::InternalServerError()
                                                .body("cannot update cart staus")
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                return HttpResponse::InternalServerError()
                                    .body("cannot update cart staus")
                            }
                        }
                    }
                    None => {
                        error!("no payments found.");
                        return HttpResponse::BadRequest().body(format!("error capturing payment"));
                    }
                },
                Err(e) => {
                    return HttpResponse::BadRequest()
                        .body(format!("error capturing payment \n {}", e));
                }
            };
            return HttpResponse::BadRequest().body(format!("error capturing payment"));
        }
    }
}

#[get("/capture/{cart_id}")]
async fn capture_payment_by_id(
    app_state: web::Data<AppState>,
    input_id: web::Path<i32>,
) -> HttpResponse {
    let id = input_id.into_inner();
    return capture_payment(app_state, id).await;
}



async fn capture_payment(app_state: web::Data<AppState>, id: i32) -> HttpResponse {
    match Payment::get_by_id(id, &app_state.db).await {
        Ok(payment) => {
            if payment.is_none() {
                return HttpResponse::InternalServerError()
                    .body("error while capturing payment - no payment found");
            }
            let payment = payment.unwrap();

            match app_state
                .http_client
                .client
                .post(format!(
                    "https://api.yookassa.ru/v3/payments/{}/capture",
                    payment.payment_id
                ))
                .basic_auth(
                    &app_state.http_client.store_id,
                    Some(&app_state.http_client.store_key),
                )
                .header("Idempotence-Key", Uuid::new_v4().to_string())
                .header("CONTENT_TYPE", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let res: PaymentDoneResponse =
                            response.json::<PaymentDoneResponse>().await.unwrap();
                        debug!("success payment capture");
                        match update_cart_status(&app_state.db, id).await {
                            Some(_) => {
                                let edited_response = json!({"id": res.id, "status":res.status,"amount": format!("{} {}", res.amount.value, res.amount.currency), "captured_at": res.captured_at, "payment_method": res.payment_method});

                                return HttpResponse::PermanentRedirect().append_header(("location", "http://localhost:8081/payment-success")).finish();
                            }
                            None => {
                                return HttpResponse::InternalServerError()
                                    .body("cannot update cart staus")
                            }
                        }
                    } else {
                        return HttpResponse::BadRequest().body(format!(
                            "error capturing payment \n {} {}",
                            response.status(),
                            response.text().await.unwrap()
                        ));
                    }
                }
                Err(e) => {
                    HttpResponse::BadRequest().body(format!("error capturing payment \n {}", e))
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("error capturing payment \n {}", e)),
    }
}

async fn update_cart_status(connetion: &sqlx::PgPool, cart_id: i32) -> Option<Cart> {
    match InsertCart::update_status_by_id(connetion, cart_id, true).await {
        Ok(val) => return Some(val),
        Err(_) => return None,
    }
}

pub async fn create_payment(
    app_state: web::Data<AppState>,
    insert_cart: InsertCart,
) -> HttpResponse {
    match insert_cart.get_cost(&app_state.db).await {
        Ok(cost) => {
            match app_state
                .http_client
                .client
                .post("https://api.yookassa.ru/v3/payments")
                .basic_auth(
                    &app_state.http_client.store_id,
                    Some(&app_state.http_client.store_key),
                )
                .header("Idempotence-Key", Uuid::new_v4().to_string())
                .header("CONTENT_TYPE", "application/json")
                .json(&PaymentRequest {
                    amount: Amount {
                        value: cost,
                        currency: "RUB".to_owned(),
                    },
                    confirmation: Confirmation {
                        confirmation_type: "redirect".to_owned(),
                        return_url: format!(
                            "http://localhost:8090/api/v1/payments/capture/{}",
                            insert_cart.cart_info.id.unwrap_or_default()
                        )
                        .to_owned(), //TODO add value
                    },
                    description: format!(
                        "Заказ №{}",
                        &insert_cart.cart_info.id.unwrap_or_default()
                    ),
                })
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let res: PaymentResponse =
                            response.json::<PaymentResponse>().await.unwrap();
                        debug!("success payment creation");
                        let payment = Payment {
                            cart_id: insert_cart.cart_info.id.unwrap_or_default(),
                            payment_id: res.id.as_str().to_owned(),
                        };
                        match payment.insert(&app_state.db).await {
                            Ok(_) => {
                                return HttpResponse::TemporaryRedirect()
                                    .body(res.confirmation.confirmation_url)
                            }
                            Err(e) => error!("err updating status (db) {}", e),
                        }
                        return HttpResponse::TemporaryRedirect()
                            .body(res.confirmation.confirmation_url);
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::BadRequest().body(format!("{}", e));
                }
            };
            return HttpResponse::InternalServerError()
                .body(format!("{}", "Error while calculating final cost"));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body(format!("{}", "Error while calculating final cost"))
        }
    }
}
