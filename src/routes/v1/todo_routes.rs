use crate::controllers::todo_controller::TodoController as controller;
use crate::middlewares::auth_middleware::auth_validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route(
                "",
                web::post()
                    .to(controller::create_todo)
                    .wrap(HttpAuthentication::bearer(auth_validator)),
            )
            .route("", web::get().to(controller::get_all_todos))
            .route("/{id}", web::get().to(controller::get_todo))
            .route("/{id}", web::put().to(controller::update_todo))
            .route("/{id}", web::delete().to(controller::delete_todo))
            .route("/user/{user_id}", web::get().to(controller::get_user_todos)),
    );
}
