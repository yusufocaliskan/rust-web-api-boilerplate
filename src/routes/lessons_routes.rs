use crate::controllers::lessons_controller as controller;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lessons")
            .route("get_all", web::get().to(controller::find_all))
            .route("/delete_all", web::get().to(controller::delete_all)),
    );
}
