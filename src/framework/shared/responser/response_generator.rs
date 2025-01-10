use crate::framework::shared::responser::responser_models::ResponserBodyGenerator;
use actix_web::HttpResponse;
use serde::Serialize;

pub struct ResponseGenerator {}

impl ResponseGenerator {
    pub fn success<T: Serialize>(message: String, data: T, status_code: u16) -> HttpResponse {
        let body = ResponserBodyGenerator::new(message, data, true, status_code);
        HttpResponse::Ok().json(body)
    }

    pub fn error<T: Serialize>(message: String, data: T, status_code: u16) -> HttpResponse {
        let body = ResponserBodyGenerator::new(message, data, true, status_code);

        HttpResponse::InternalServerError().json(body)
    }
}
