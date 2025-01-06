use crate::configs::Configs;
use colored::*;
use mongodb::{options::ClientOptions, Database};
use std::sync::Arc;

pub async fn establish_database_connection(
    configs: &Arc<Configs>,
) -> Result<Database, mongodb::error::Error> {
    let db_name = configs
        .get("MONGO_DB_NAME")
        .expect("MONGO_DB_NAME env var is missing");
    let db_url = configs
        .get("MONGO_DB_URL")
        .expect("MONGO_DB_URL env var is missing");

    println!("{} {}", "Connecting to MongoDB...".blue(), db_url.yellow());

    // Attempt to connect
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

    let client = match mongodb::Client::with_options(client_options) {
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

    // Get the database
    let database = client.database(&db_name);
    println!(
        "{} {}",
        "--> Successfully connected to the database 🚀:".green(),
        db_name.yellow()
    );

    Ok(database)
}
