use crate::controllers::user_controller::UserController as controller;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route(
                "/get_an_object_id",
                web::get().to(controller::get_an_object_id),
            )
            .route("/create", web::post().to(controller::create_user))
            .route("/find-all", web::get().to(controller::find_all))
            .route("/delete-all", web::get().to(controller::delete_by_id)),
    );
}
