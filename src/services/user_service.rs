use mongodb::Database;

#[derive(Clone)]
pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all_user_service(&self) {
        self.db.create_collection("helloo").await.expect("Panicccc");
    }
}
