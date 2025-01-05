use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Database;
use std::env;

pub async fn establish_database_connection() -> Result<Database, Error> {
    let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env var");
    let db_url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL env var");

    let client_options = ClientOptions::parse(db_url)
        .await
        .expect("Error creating a client options");
    let client = mongodb::Client::with_options(client_options).expect("Mongo Client Failure");
    Ok(client.database(&db_name))
}
