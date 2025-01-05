use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Load the environments
    dotenv::dotenv().expect("------ .env bar nebu!.------");
    env_logger::init();

    //Start the server
    start_server().await
}

//Build the server
async fn start_server() -> std::io::Result<()> {
    let server_address = env::var("SERVER_ADDRESS").expect("ADDRESS env variable not set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT env variable not set");
    let app_url = format!("{}:{}", &server_address, &server_port);

    HttpServer::new(move || {
        App::new()
            .wrap(get_cors_configurations())
            .service(actix_web::web::scope("/api/v1").configure(routes::init_routes))
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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Silav u rez")
}
