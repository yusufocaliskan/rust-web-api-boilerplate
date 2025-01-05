use actix_web::{HttpResponse, Responder};

pub struct LessonController {}

impl LessonController {
    pub async fn find_all() -> impl Responder {
        HttpResponse::Ok().body("Cawaniko: Silav Lessons")
    }
    pub async fn delete_all() -> impl Responder {
        HttpResponse::Ok().body("Lessons:: Deleted all")
    }
}
