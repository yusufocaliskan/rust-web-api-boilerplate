use crate::controllers::user_controller as controller;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("hello", web::get().to(controller::find_all)));
}
