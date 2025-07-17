use crate::models::user::User;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, results::InsertOneResult, Database};

const COLLECTION_NAME: &str = "users";

pub async fn create(db: &Database, user: &User) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<User>(COLLECTION_NAME);
    collection.insert_one(user, None).await
}

pub async fn find_by_username(db: &Database, username: &str) -> Result<Option<User>, Error> {
    let collection = db.collection::<User>(COLLECTION_NAME);
    collection.find_one(doc! { "username": username }, None).await
}

pub async fn get_by_id(db: &Database, id: ObjectId) -> Result<Option<User>, Error> {
    let collection = db.collection::<User>(COLLECTION_NAME);
    collection.find_one(doc! { "_id": id }, None).await
} 