use crate::framework::database::{get_database, MongoPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    pub db_pool: Arc<MongoPool>,
}

impl UserService {
    pub fn new(db_pool: Arc<MongoPool>) -> Self {
        Self { db_pool }
    }

    pub async fn find_all_user_service(&self) {
        let db = get_database(self.db_pool.clone()).await;
        db.create_collection("Vira--------helloo")
            .await
            .expect("Panicccc");
    }
}
