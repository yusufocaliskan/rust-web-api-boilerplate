use crate::models::auth_model::TokenClaims;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    match decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("test-key".as_ref()),
        &Validation::default(),
    ) {
        Ok(_claims) => Ok(req),
        Err(e) => {
            let error = ErrorUnauthorized(e.to_string());
            Err((error, req))
        }
    }
}
