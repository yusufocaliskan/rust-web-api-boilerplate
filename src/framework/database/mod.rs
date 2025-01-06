use bson::doc;
use colored::*;
use mobc::{async_trait, Connection, Pool};
use mongodb::Client;
use mongodb::{options::ClientOptions, Database};
use std::env;
use std::sync::Arc;

//create types
pub type MongoConnection = Connection<MongoConnectionManager>;
pub type MongoPool = Pool<MongoConnectionManager>;

#[derive(Clone)]
pub struct MongoConnectionManager {
    db_url: String,
}

#[async_trait]
impl mobc::Manager for MongoConnectionManager {
    type Connection = Client;
    type Error = mongodb::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let db_url = self.db_url.clone();

        let client_options = match ClientOptions::parse(db_url).await {
            Ok(options) => options,
            Err(err) => {
                eprintln!(
                    "{}: {}",
                    "Failed to create client options".red(),
                    err.to_string().red()
                );
                return Err(err);
            }
        };

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(err) => {
                eprintln!(
                    "{}: {}",
                    "--> Failed to create MongoDB client ❤️‍🩹".red(),
                    err.to_string().red()
                );
                return Err(err);
            }
        };
        Ok(client)
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let database = conn.database("ziwonify");
        let result = database.run_command(doc! { "ping": 1 }).await;

        if result.is_err() {
            eprintln!(
                "{}: {}",
                "Failed to ping MongoDB".red(),
                result.unwrap_err().to_string().red()
            );
        } else {
            println!("{}", "MongoDB connection successful!".green());
        }
        Ok(conn)
    }
}

pub fn create_mongo_pool() -> MongoPool {
    let db_url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL env variable not set");

    let manager = MongoConnectionManager {
        db_url: String::from(&db_url),
    };

    //Get max oonpne aconnection size from .env
    Pool::builder().max_open(50).build(manager)
}

pub async fn get_database(pool: Arc<MongoPool>) -> Database {
    let db_name = dotenv::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env variable not set");
    let connection = pool.get().await.expect("Failed to get connection");
    connection.database(&db_name)
}
