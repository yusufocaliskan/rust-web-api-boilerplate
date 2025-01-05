use crate::services::lesson_service::LessonService;
use crate::services::user_service::UserService;
use mongodb::Database;

pub mod lesson_service;
pub mod user_service;

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_service: UserService,
    pub lessons_service: LessonService,
}

impl ServiceContainer {
    pub fn new(db: Database) -> Self {
        Self {
            user_service: UserService::new(db.clone()),
            lessons_service: LessonService::new(db.clone()),
        }
    }
}
