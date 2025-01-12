use crate::controllers::user_controller::UserController as controller;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(controller::create_user))
            .route("/all", web::get().to(controller::get_all_users))
            .route("/{id}", web::get().to(controller::get_user))
            .route("/{id}", web::put().to(controller::update_user))
            .route("/{id}", web::delete().to(controller::delete_user)),
    );
}
