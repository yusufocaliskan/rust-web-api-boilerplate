use crate::framework::database::IDatabase;
use crate::modules::AppModules;
use crate::services::lessons_services::ILessonsService;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use shaku_actix::Inject;

pub struct LessonController {}

impl LessonController {
    pub async fn find_all(
        state: web::Data<AppState>,
        lessons_services: Inject<AppModules, dyn ILessonsService>,
        database: Inject<AppModules, dyn IDatabase>,
    ) -> impl Responder {
        // state.services.lessons_service.find_all_lessons()
        lessons_services.lessons();
        // database.get_db().collection("432432")
        HttpResponse::Ok().body("Hello::find-all")
    }
}
