use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub struct UserController {}
impl UserController {
    pub async fn find_all(state: web::Data<AppState>) -> impl Responder {
        state
            .db
            .create_collection("silavooo")
            .await
            .expect("TODO: panic message");

        HttpResponse::Ok().body(format!("Hello, find_all {}", state.app_name))
    }

    pub async fn delete_all() -> impl Responder {
        HttpResponse::Ok().body("Deleted all")
    }
}
