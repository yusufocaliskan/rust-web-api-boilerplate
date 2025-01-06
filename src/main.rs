extern crate mongodb;
use crate::configs::Configs;
use crate::framework::database::{create_mongo_pool, MongoPool};
use crate::services::ServiceContainer;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
use std::sync::Arc;

mod configs;
mod controllers;
mod framework;
mod routes;
mod services;

#[derive(Clone)]
struct AppState {
    app_name: String,
    services: Arc<ServiceContainer>,
    configs: Arc<Configs>,
    db_pool: Arc<MongoPool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Load the environments
    dotenv::dotenv().expect("------ .env bar nebu!.------");
    env_logger::init();

    //displayes logs
    env::set_var("RUST_LOG", "info");

    //Configurations
    let configs = Arc::new(Configs::new());

    //Db connection
    let db_pool = Arc::new(create_mongo_pool());

    // let service_container = ServiceContainer::new(db_connection.clone());
    let service_container = Arc::new(ServiceContainer::new(db_pool.clone()));

    //app states
    let states = web::Data::new(AppState {
        app_name: String::from("App Name"),
        services: service_container,
        configs,
        db_pool,
    });

    //Start the server
    start_server(states).await
}

//Build the server
async fn start_server(states: web::Data<AppState>) -> std::io::Result<()> {
    //get envs
    let server_address = env::var("SERVER_ADDRESS").expect("ADDRESS env variable not set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT env variable not set");
    let app_url = format!("{}:{}", &server_address, &server_port);

    HttpServer::new(move || {
        App::new()
            //cors settings
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(get_cors_configurations())
            .app_data(states.clone())
            //set routes
            .service(web::scope("/api/v1").configure(routes::v1::init_routes))
    })
    .bind(&app_url)?
    .run()
    .await
}

//Cors configurations
fn get_cors_configurations() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        // .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
