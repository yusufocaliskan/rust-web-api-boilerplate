use crate::controllers::lessons_controller::LessonController as controller;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/lessons").route("/find-all", web::get().to(controller::find_all)));
}
