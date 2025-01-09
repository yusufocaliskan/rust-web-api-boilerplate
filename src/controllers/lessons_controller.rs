use crate::services::ServiceContainer;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct LessonController {
    services: Arc<ServiceContainer>,
}

impl LessonController {
    pub fn new(services: Arc<ServiceContainer>) -> Self {
        Self { services }
    }
    pub async fn find_all(state: web::Data<AppState>) -> impl Responder {
        HttpResponse::Ok().body("Hello:: find-all")
    }
}
