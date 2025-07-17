// backend/src/persistence/db.rs

use mongodb::{Client, Database};
use std::env;

pub async fn get_db() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://mongo:27017".to_string());
    let client = Client::with_uri_str(&mongo_uri).await?;
    Ok(client.database("volleyapp"))
} 