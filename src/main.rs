extern crate mongodb;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use mongodb::Database;
use std::env;

mod controllers;
mod framework;
mod routes;
mod services;

#[derive(Clone)]
struct AppState {
    app_name: String,
    db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Load the environments
    dotenv::dotenv().expect("------ .env bar nebu!.------");
    env_logger::init();

    //Db connection
    let db = framework::database::establish_database_connection().await;

    let db_connection = match db {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Unable to connect to MongoDB: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ));
        }
    };

    //app states
    let states = web::Data::new(AppState {
        app_name: String::from("App Name"),
        db: db_connection.clone(),
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
            .wrap(get_cors_configurations())
            .app_data(states.clone())
            //set routes
            .service(actix_web::web::scope("/api/v1").configure(routes::v1::init_routes))
        // .service(actix_web::web::scope("/api/v2").configure(routes::v2::init_routes))
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
