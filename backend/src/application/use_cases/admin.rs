// backend/src/application/use_cases/admin.rs

use crate::domain::models::user::User;
use crate::presentation::dto::user::{UserImportRecord, UserRole};
use crate::infrastructure::persistence::repositories::user_repository;
use csv::ReaderBuilder;
use std::io::Cursor;
use mongodb::bson::doc;
use mongodb::Database;
use serde::de::Error;

pub struct AdminUseCases;

impl AdminUseCases {
    pub async fn import_users_from_csv(
        db: &Database,
        csv_data: &[u8],
    ) -> Result<usize, String> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        let mut imported_count = 0;
        for result in reader.deserialize::<UserImportRecord>() {
            match result {
                Ok(record) => {
                    // Since email is not in the User model, we'll check by username for duplicates.
                    let existing_user = user_repository::find_by_username(db, &record.username)
                        .await
                        .map_err(|e| e.to_string())?;

                    if existing_user.is_none() {
                        let user = User {
                            id: None,
                            username: record.username,
                            password_hash: "".to_string(), // You should handle password hashing
                            role: record.role.map_or(crate::domain::models::user::UserRole::User, |r| match r {
                                UserRole::Admin => crate::domain::models::user::UserRole::Admin,
                                UserRole::User => crate::domain::models::user::UserRole::User,
                            }),
                            rating: record.rating,
                            telegram_id: record.telegram_id.map(|id| id.to_string()),
                            instagram_link: record.instagram_link,
                            image_url: None,
                            subscriptions: vec![],
                        };
                        user_repository::create(db, &user)
                            .await
                            .map_err(|e| e.to_string())?;
                        imported_count += 1;
                    }
                }
                Err(e) => return Err(format!("Failed to parse CSV record: {}", e)),
            }
        }
        Ok(imported_count)
    }

    pub async fn import_users_from_json(db: &Database, json_data: &[u8]) -> Result<usize, String> {
        let records: Vec<UserImportRecord> = serde_json::from_slice(json_data)
            .map_err(|e| e.to_string())?;
        
        let mut count = 0;
        for record in records {
            let existing_user = user_repository::find_by_username(db, &record.username)
                .await
                .map_err(|e| e.to_string())?;

            if existing_user.is_none() {
                let user = User {
                    id: None,
                    username: record.username,
                    password_hash: "".to_string(), // You should handle password hashing
                    role: record.role.map_or(crate::domain::models::user::UserRole::User, |r| match r {
                        UserRole::Admin => crate::domain::models::user::UserRole::Admin,
                        UserRole::User => crate::domain::models::user::UserRole::User,
                    }),
                    rating: record.rating,
                    telegram_id: record.telegram_id.map(|id| id.to_string()),
                    instagram_link: record.instagram_link,
                    ..Default::default()
                };
                user_repository::create(db, &user)
                    .await
                    .map_err(|e| e.to_string())?;
                count += 1;
            }
        }
        Ok(count)
    }
} 