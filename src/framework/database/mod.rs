use mongodb::{options::ClientOptions, Client, Database};
use std::env;

#[derive(Clone)]
pub struct DatabaseInstance {
    pub client: Client,
    pub db_name: String,
}

impl DatabaseInstance {
    pub async fn new() -> Self {
        let db_url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL env variable not set");
        let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env variable not set");

        let client_options = ClientOptions::parse(&db_url)
            .await
            .expect("Failed to parse MongoDB connection string");

        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

        println!("MongoDB connection established!");

        Self { client, db_name }
    }

    pub fn get_db(&self) -> Database {
        self.client.database(&self.db_name)
    }
}
