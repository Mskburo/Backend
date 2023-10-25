use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenType {
    Refresh,
    Access,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub login: String,
    pub token_type: TokenType,
    pub exp: usize,
}

impl TokenClaims {
    pub fn generate_access(user_id: String) -> String {
        TokenClaims::generate_token(user_id, TokenType::Access)
    }

    pub fn generate_refresh(user_id: String) -> String {
        TokenClaims::generate_token(user_id, TokenType::Refresh)
    }

    fn generate_token(login: String, token_type: TokenType) -> String {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
        .unwrap();
        let claims = TokenClaims {
            login,
            token_type: token_type.clone(),
            exp: match token_type {
                TokenType::Access => (Utc::now() + Duration::minutes(5)).timestamp() as usize,
                TokenType::Refresh => (Utc::now() + Duration::days(30)).timestamp() as usize,
            },
        };
        claims.sign_with_key(&jwt_secret).unwrap()
    }
    pub fn get_token_claims(token_string: &str) -> Result<TokenClaims, &str> {
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        token_string
            .verify_with_key(&key)
            .map_err(|_| "Invalid token")
    }
}
