use crate::framework::database::MongoPool;
use crate::services::lesson_service::LessonService;
use crate::services::user_service::UserService;
use std::sync::Arc;

pub mod lesson_service;
pub mod user_service;

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_service: UserService,
    pub lessons_service: LessonService,
}

impl ServiceContainer {
    pub fn new(db_pool: Arc<MongoPool>) -> Self {
        Self {
            user_service: UserService::new(db_pool.clone()),
            lessons_service: LessonService::new(db_pool.clone()),
        }
    }
}
