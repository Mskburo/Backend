use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use tracing::warn;

pub async fn new_user_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = std::env::var("TOKEN").expect("TOKEN must be set");
    if credentials.token() == token {
        Ok(req)
    } else {
        let config = req
            .app_data::<bearer::Config>()
            .cloned()
            .unwrap_or_default()
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
        warn!("{:?}",  req);
        Err((AuthenticationError::from(config).into(), req))
    }
}

use chrono::Utc;

use crate::token::{TokenClaims, TokenType};

pub async fn validator_refresh(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    validator(req, credentials, TokenType::Refresh).await
}
pub async fn validator_acces(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    validator(req, credentials, TokenType::Access).await
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
    token_type: TokenType,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match TokenClaims::get_token_claims(credentials.token()) {
        Ok(value) => {
            if value.token_type != token_type {
               warn!("{:?}",  req);
                return Err((
                    AuthenticationError::from(bearer::Config::default()).into(),
                    req,
                ));
            }

            if value.exp < Utc::now().timestamp() as usize {
                warn!("{:?}",  req);
                return Err((
                    AuthenticationError::from(bearer::Config::default()).into(),
                    req,
                ));
            }
            req.extensions_mut().insert(value);

            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
           warn!("{:?}",  req);
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
