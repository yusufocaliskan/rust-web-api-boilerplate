use actix_web::HttpResponse;
pub struct ResponseGenerator {}

impl ResponseGenerator {
    pub fn success(message: String) -> HttpResponse {
        HttpResponse::Ok().body(message)
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError().body(message)
    }
}
