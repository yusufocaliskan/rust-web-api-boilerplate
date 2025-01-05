use actix_web::{HttpResponse, Responder};

pub async fn find_all() -> impl Responder {
    HttpResponse::Ok().body("Lesssons: Hello, find_all")
}

pub async fn delete_all() -> impl Responder {
    HttpResponse::Ok().body("Lessons:: Deleted all")
}
