// backend/src/application/use_cases/admin.rs

use crate::domain::User;
use crate::infrastructure::persistence::repositories::user_repository;
use crate::presentation::dto::user::UserImportRecord;
use csv::ReaderBuilder;
use sqlx::PgConnection;
use std::io::Cursor;

pub struct AdminUseCases;

impl AdminUseCases {
    pub async fn import_users_from_csv(conn: &mut PgConnection, csv_data: &[u8]) -> Result<usize, String> {
        let mut rdr = ReaderBuilder::new().from_reader(Cursor::new(csv_data));
        let mut count = 0;
        for result in rdr.deserialize() {
            let record: UserImportRecord = result.map_err(|e| e.to_string())?;

            let existing_user = user_repository::find_by_username(conn, &record.username)
                .await
                .map_err(|e| e.to_string())?;

            if existing_user.is_none() {
                let mut user = User::new(record.username.clone(), "".to_string());
                user.role = record.role.map_or(crate::domain::UserRole::User, |r| match r {
                    crate::presentation::dto::user::UserRole::Admin => crate::domain::UserRole::Admin,
                    crate::presentation::dto::user::UserRole::User => crate::domain::UserRole::User,
                });
                user.rating = record.rating;
                user.telegram_id = record.telegram_id.map(|id| id.to_string());
                user.instagram_link = record.instagram_link;
                user.subscribed = false;

                user_repository::create(conn, &user)
                    .await
                    .map_err(|e| e.to_string())?;
                count += 1;
            }
        }
        Ok(count)
    }

    pub async fn import_users_from_json(conn: &mut PgConnection, json_data: &[u8]) -> Result<usize, String> {
        let records: Vec<UserImportRecord> =
            serde_json::from_slice(json_data).map_err(|e| e.to_string())?;

        let mut count = 0;
        for record in records {
            let existing_user = user_repository::find_by_username(conn, &record.username)
                .await
                .map_err(|e| e.to_string())?;

            if existing_user.is_none() {
                let mut user = User::new(record.username.clone(), "".to_string());
                user.role = record.role.map_or(crate::domain::UserRole::User, |r| match r {
                    crate::presentation::dto::user::UserRole::Admin => crate::domain::UserRole::Admin,
                    crate::presentation::dto::user::UserRole::User => crate::domain::UserRole::User,
                });
                user.rating = record.rating;
                user.telegram_id = record.telegram_id.map(|id| id.to_string());
                user.instagram_link = record.instagram_link;
                user.subscribed = false;
                
                user_repository::create(conn, &user)
                    .await
                    .map_err(|e| e.to_string())?;
                count += 1;
            }
        }
        Ok(count)
    }
} 