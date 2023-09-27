use actix_web::{get, post, web, HttpResponse};

use serde_json::json;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{
    models::{
        cart::InsertCart,
        payments::{
            done_response::PaymentDoneResponse,
            request::{Amount, Confirmation, PaymentRequest},
            response::PaymentResponse,
            Payment,
        },
    },
    AppState,
};

#[post("/capture")]
async fn capture_webhook_event(
    app_state: web::Data<AppState>,
    input_id: web::Path<i32>,
) -> HttpResponse {
    let id = input_id.into_inner();
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
                        debug!("success payment creation");

                        return HttpResponse::Accepted().json(res);
                    }
                    return HttpResponse::BadRequest().body(format!("error capturing payment."));
                }
                Err(e) => {
                    HttpResponse::BadRequest().body(format!("error capturing payment \n {}", e))
                }
            }
        }
        Err(_) => todo!(),
    }
}

#[get("/capture/{cart_id}")]
async fn capture_payment(app_state: web::Data<AppState>, input_id: web::Path<i32>) -> HttpResponse {
    let id = input_id.into_inner();
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
                        match InsertCart::update_status_by_id(&app_state.db, payment.cart_id, true)
                            .await
                        {
                            Ok(is_done) => {
                                if is_done {
                                    let edited_response = json!({"id": res.id, "status":res.status,"amount": format!("{} {}", res.amount.value, res.amount.currency), "captured_at": res.captured_at, "payment_method": res.payment_method});
                                    return HttpResponse::Accepted().json(edited_response);
                                } else {
                                    error!("error updating status");
                                };
                            }
                            Err(e) => error!("err updating status (db) {}", e),
                        }
                        return HttpResponse::Accepted().json(res);
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
