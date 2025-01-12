use actix_web::web::ServiceConfig;

pub mod user_routes;

pub fn init_routes(cfg: &mut ServiceConfig) {
    user_routes::init(cfg);
    //lessons_routes::init(cfg);
}
