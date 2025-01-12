use actix_web::web::ServiceConfig;

pub mod todo_routes;
pub mod user_routes;

pub fn init_routes(cfg: &mut ServiceConfig) {
    user_routes::init(cfg);
    todo_routes::init(cfg);
}
