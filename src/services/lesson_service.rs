use crate::framework::database::{get_database, MongoPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct LessonService {
    db_pool: Arc<MongoPool>,
}

impl LessonService {
    pub fn new(db_pool: Arc<MongoPool>) -> Self {
        Self { db_pool }
    }

    pub async fn find_all_lessons(&self) {
        let db = get_database(self.db_pool.clone()).await;
        db.create_collection("find_all_lessons")
            .await
            .expect("TODO: panic message");
    }
}
