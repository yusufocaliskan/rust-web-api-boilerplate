use crate::services::ServiceContainer;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct UserController {
    services: Arc<ServiceContainer>,
}

impl UserController {
    pub fn new(services: Arc<ServiceContainer>) -> Self {
        Self { services }
    }

    pub async fn find_all(state: web::Data<AppState>) -> impl Responder {
        // HTTP response'u hemen döndür
        HttpResponse::Ok().body(format!("Hello, {}", state.app_name))
    }

    pub async fn delete_by_id(state: web::Data<AppState>) -> impl Responder {
        // println!("Silav deleted {:#?}", Self::find_by_user_id());
        HttpResponse::Ok().body("Deleted all")
    }
}
