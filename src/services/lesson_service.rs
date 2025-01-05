use mongodb::Database;

#[derive(Clone)]
pub struct LessonService {
    db: Database,
}

impl LessonService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all_lessons(&self) {
        self.db
            .create_collection("find_all_lessons")
            .await
            .expect("TODO: panic message");
    }
}
