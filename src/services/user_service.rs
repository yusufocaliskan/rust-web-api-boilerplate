use mongodb::Database;

#[derive(Clone)]
pub struct UserService {
    pub database: Database,
}

impl UserService {
    pub async fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn find_all_user_service(&self) {
        self.database
            .create_collection("----- >> testlooooo")
            .await
            .expect("TODO: panic message");
    }
}
