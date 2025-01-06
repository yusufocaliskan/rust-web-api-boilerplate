use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub struct UserController {}

impl UserController {
    pub async fn find_all(state: web::Data<AppState>) -> impl Responder {
        //services.
        state.services.user_service.find_all_user_service().await;
        state.services.lessons_service.find_all_lessons().await;

        //Using tokens.
        let token = state.configs.get("MONGO_DB_PASSWORD").expect("Error");
        println!("User Service Token: {}", token);
        HttpResponse::Ok().body(format!("Hello, {}", state.app_name))
    }

    pub async fn delete_by_id(state: web::Data<AppState>) -> impl Responder {
        // println!("Silav deleted {:#?}", Self::find_by_user_id());
        HttpResponse::Ok().body("Deleted all")
    }
}
