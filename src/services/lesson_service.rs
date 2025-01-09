use mongodb::Database;

#[derive(Clone)]
pub struct LessonService {
    pub database: Database,
}

impl LessonService {
    pub async fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn find_all_lessons(&self, name: &str) {
        self.database
            .create_collection(name)
            .await
            .expect("TODO: panic message");
    }
}
