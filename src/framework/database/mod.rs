use async_trait::async_trait;
use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client, Database};
use shaku::{Component, Interface};
use std::env;

#[async_trait]
pub trait IDatabaseService: Interface {
    async fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_database(&self) -> Database;
    fn get_client(&self) -> Client;
}

#[derive(Component)]
#[shaku(interface = IDatabaseService)]
pub struct DatabaseService {
    database: Database,
    client: Client,
}

impl DatabaseService {
    async fn new_internal() -> Result<Self, Error> {
        let db_url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL env variable not set");
        let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env variable not set");

        let client_options = ClientOptions::parse(&db_url).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&db_name);

        println!("MongoDB connection established!");
        Ok(Self { database, client })
    }
}

#[async_trait]
impl IDatabaseService for DatabaseService {
    async fn new() -> Result<Self, Error> {
        Self::new_internal().await
    }

    fn get_database(&self) -> Database {
        self.database.clone()
    }

    fn get_client(&self) -> Client {
        self.client.clone()
    }
}
