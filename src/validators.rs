use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, self}};



use actix_web::{dev::ServiceRequest, Error};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = std::env::var("TOKEN").expect("TOKEN must be set");
    if credentials.token() == token {
        Ok(req)
    } else {
        let config = req.app_data::<bearer::Config>()
            .cloned()
            .unwrap_or_default()
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err((AuthenticationError::from(config).into(), req))
    }
}