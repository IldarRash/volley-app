use crate::domain::User;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, results::{InsertOneResult, UpdateResult, DeleteResult}, Database};
use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;

const COLLECTION_NAME: &str = "users";

#[derive(Debug, Serialize, Deserialize)]
struct UserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub rating: f64,
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
    pub image_url: Option<String>,
    pub subscriptions: Vec<serde_json::Value>,
}

impl From<&User> for UserDocument {
    fn from(user: &User) -> Self {
        UserDocument {
            id: user.id,
            username: user.username.clone(),
            password_hash: user.password_hash.clone(),
            role: serde_json::to_string(&user.role).unwrap().trim_matches('"').to_string(),
            rating: user.rating,
            telegram_id: user.telegram_id.clone(),
            instagram_link: user.instagram_link.clone(),
            image_url: user.image_url.clone(),
            subscriptions: user.subscriptions.iter().map(|s| serde_json::to_value(s).unwrap()).collect(),
        }
    }
}

impl From<UserDocument> for User {
    fn from(doc: UserDocument) -> Self {
        User {
            id: doc.id,
            username: doc.username,
            password_hash: doc.password_hash,
            role: serde_json::from_str(&format!("\"{}\"", doc.role)).unwrap(),
            rating: doc.rating,
            telegram_id: doc.telegram_id,
            instagram_link: doc.instagram_link,
            image_url: doc.image_url,
            subscriptions: doc.subscriptions.into_iter().map(|s| serde_json::from_value(s).unwrap()).collect(),
        }
    }
}

pub async fn create(db: &Database, user: &User) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let doc = UserDocument::from(user);
    collection.insert_one(doc, None).await
}

pub async fn find_by_username(db: &Database, username: &str) -> Result<Option<User>, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let doc = collection.find_one(doc! { "username": username }, None).await?;
    Ok(doc.map(User::from))
}

pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Option<User>, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let doc = collection.find_one(doc! { "_id": id }, None).await?;
    Ok(doc.map(User::from))
}

pub async fn find_all(db: &Database) -> Result<Vec<User>, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let mut cursor = collection.find(doc! {}, None).await?;
    let mut users = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        users.push(User::from(doc));
    }
    Ok(users)
}

pub async fn update(db: &Database, id: &ObjectId, user: &User) -> Result<UpdateResult, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let doc = UserDocument::from(user);
    collection.replace_one(doc! { "_id": id }, doc, None).await
}

pub async fn update_password(db: &Database, id: &ObjectId, password_hash: &str) -> Result<(), Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    let filter = doc! { "_id": id };
    let update = doc! { "$set": { "password_hash": password_hash } };
    
    collection.update_one(filter, update, None).await?;
    Ok(())
}

pub async fn delete(db: &Database, id: &ObjectId) -> Result<DeleteResult, Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    collection.delete_one(doc! { "_id": id }, None).await
}

pub async fn clear(db: &Database) -> Result<(), Error> {
    let collection = db.collection::<UserDocument>(COLLECTION_NAME);
    collection.delete_many(doc! {}, None).await?;
    Ok(())
} 