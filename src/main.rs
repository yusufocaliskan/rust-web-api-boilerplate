extern crate mongodb;
use crate::configs::Configs;
use crate::framework::database::{
    establish_db_connection, DatabaseInstance, DatabaseInstanceParameters,
};
use crate::modules::AppModules;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
use std::sync::Arc;

mod configs;
mod controllers;
mod framework;
mod modules;
mod repositories;
mod routes;
mod services;

#[derive(Clone)]
struct AppState {
    app_name: String,
    configs: Arc<Configs>,
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

    //db connection
    let database = establish_db_connection().await;

    let modules = Arc::new(
        AppModules::builder()
            .with_component_parameters::<DatabaseInstance>(DatabaseInstanceParameters {
                database: database.clone(),
            })
            .build(),
    );
    /*.with_component_parameters::<DatabaseImpl>(DatabaseImplParameters {
        db: database, // Bu alan struct DatabaseImplParameters (otomatik jenerik) ile eşleşir
    })*/
    //app states
    let states = web::Data::new(AppState {
        app_name: String::from("App Name"),
        configs,
    });

    //Start the server
    //get envs
    let server_address = env::var("SERVER_ADDRESS").expect("ADDRESS env variable not set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT env variable not set");
    let app_url = format!("{}:{}", &server_address, &server_port);

    HttpServer::new(move || {
        App::new()
            //cors settings
            .wrap(Logger::default())
            .app_data(states.clone())
            .app_data(modules.clone())
            .wrap(middleware::Compress::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    // .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            //set routes
            .service(web::scope("/api/v1").configure(routes::v1::init_routes))
    })
    .bind(&app_url)?
    .run()
    .await
}
