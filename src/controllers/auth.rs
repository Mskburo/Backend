use crate::{token::TokenClaims, AppState};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};
use tracing::debug;

use crate::models::user::*;
use sqlx;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[post("/register")]
async fn create_user(state: Data<AppState>, body: Json<User>) -> impl Responder {
    let mut user: User = body.into_inner();
    let hash = Argon2::default()
        .hash_password(user.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    if !user.is_unique(&state).await.unwrap() {
        return HttpResponse::BadRequest().json("login already claimed");
    }
    user.password = hash;

    match user.insert(state).await {
        Ok(_) => HttpResponse::Ok().body("success"),
        Err(error) => match error {
            sqlx::Error::Database(error) => {
                match error
                    .downcast_ref::<sqlx::postgres::PgDatabaseError>()
                    .code()
                    == "23505"
                {
                    // AAAAAA 1.5 hours to downcast_ref::<sqlx::postgres::PgDatabaseError> // HEHE Copied from my old project
                    true => HttpResponse::BadRequest().body("username or email already claimed"),
                    false => HttpResponse::InternalServerError().body(format!("{:?}", error)),
                }
            }
            _ => HttpResponse::InternalServerError().body(format!("{:?}", error)),
        },
    }
}
#[get("/")]
async fn generate_access(credentials: BearerAuth) -> HttpResponse {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    HttpResponse::Ok().body(TokenClaims::generate_access(calims.login))
}

#[post("/login")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let login = credentials.user_id();
    let pass = credentials.password();
    debug!("login {} pass:{:?}", login, pass);
    match pass {
        None => HttpResponse::Unauthorized().body("Must provide user_name and password"),
        Some(pass) => match User::get_user_by_name(login, state).await {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.password).unwrap();
                let is_valid = Argon2::default()
                    .verify_password(pass.as_bytes(), &parsed_hash)
                    .is_ok();
                if is_valid {
                    HttpResponse::Ok().body(TokenClaims::generate_refresh(user.login))
                } else {
                    HttpResponse::BadRequest().body("Incorrect user_name or password")
                }
            }
            Err(error) => HttpResponse::InternalServerError().body(format!("{:?}", error)),
        },
    }
}
