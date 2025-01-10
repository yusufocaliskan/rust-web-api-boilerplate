use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client, Database};
use shaku::{Component, Interface, Provider};
use std::env;
#[async_trait]
pub trait IDatabase: Interface {
    fn instance(&self) -> &Database;
}

#[derive(Component)]
#[shaku(interface = IDatabase)]
pub struct DatabaseInstance {
    database: Database,
}

impl IDatabase for DatabaseInstance {
    fn instance(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
pub trait IDatabaseProvider {
    async fn database(&self) -> Database;
}
#[derive(Provider)]
#[shaku(interface = IDatabaseProvider)]
pub struct DatabaseProvider {}

#[async_trait]
impl IDatabaseProvider for DatabaseProvider {
    async fn database(&self) -> Database {
        establish_db_connection().await
    }
}

pub async fn establish_db_connection() -> Database {
    let db_url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL env variable not set");
    let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env variable not set");

    let client_options = ClientOptions::parse(&db_url)
        .await
        .expect("Failed to parse MongoDB connection string");

    let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

    println!("MongoDB connection established!");
    client.database(&db_name)
}
