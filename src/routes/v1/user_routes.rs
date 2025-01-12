use crate::controllers::auth_controller::AuthController as auth_controler;
use crate::controllers::user_controller::UserController as controller;
use crate::middlewares::auth_middleware::auth_validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(auth_controler::login))
            .route("/create", web::post().to(controller::create_user))
            .route("/all", web::get().to(controller::get_all_users))
            .route("/{id}", web::get().to(controller::get_user))
            .route(
                "/{id}",
                web::put()
                    .to(controller::update_user)
                    .wrap(HttpAuthentication::bearer(auth_validator)),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(controller::delete_user)
                    .wrap(HttpAuthentication::bearer(auth_validator)),
            ),
    );
}
