use sqlx::{types::uuid::Uuid, PgConnection, Row, types::Json};
use crate::domain::{User, UserRole, Subscription};

pub async fn create(conn: &mut PgConnection, user: &User) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO users (id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed
        "#
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.role)
    .bind(&user.rating)
    .bind(&user.telegram_id)
    .bind(&user.instagram_link)
    .bind(&user.image_url)
    .bind(Json(&user.subscriptions))
    .bind(&user.subscribed)
    .fetch_one(conn)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        rating: row.get("rating"),
        telegram_id: row.get("telegram_id"),
        instagram_link: row.get("instagram_link"),
        image_url: row.get("image_url"),
        subscriptions: row.get::<Json<Vec<Subscription>>, _>("subscriptions").0,
        subscribed: row.get("subscribed"),
    })
}

pub async fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed FROM users WHERE username = $1"#
    )
    .bind(username)
    .fetch_optional(conn)
    .await?;

    Ok(row.map(|row| User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        rating: row.get("rating"),
        telegram_id: row.get("telegram_id"),
        instagram_link: row.get("instagram_link"),
        image_url: row.get("image_url"),
        subscriptions: row.get::<Json<Vec<Subscription>>, _>("subscriptions").0,
        subscribed: row.get("subscribed"),
    }))
}

pub async fn find_by_id(conn: &mut PgConnection, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed FROM users WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;

    Ok(row.map(|row| User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        rating: row.get("rating"),
        telegram_id: row.get("telegram_id"),
        instagram_link: row.get("instagram_link"),
        image_url: row.get("image_url"),
        subscriptions: row.get::<Json<Vec<Subscription>>, _>("subscriptions").0,
        subscribed: row.get("subscribed"),
    }))
}

pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed FROM users"#
    )
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        rating: row.get("rating"),
        telegram_id: row.get("telegram_id"),
        instagram_link: row.get("instagram_link"),
        image_url: row.get("image_url"),
        subscriptions: row.get::<Json<Vec<Subscription>>, _>("subscriptions").0,
        subscribed: row.get("subscribed"),
    }).collect())
}

pub async fn update(conn: &mut PgConnection, id: &Uuid, user: &User) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET username = $2, password_hash = $3, role = $4, rating = $5, telegram_id = $6, instagram_link = $7, image_url = $8, subscriptions = $9, subscribed = $10
        WHERE id = $1
        RETURNING id, username, password_hash, role, rating, telegram_id, instagram_link, image_url, subscriptions, subscribed
        "#
    )
    .bind(id)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.role)
    .bind(&user.rating)
    .bind(&user.telegram_id)
    .bind(&user.instagram_link)
    .bind(&user.image_url)
    .bind(Json(&user.subscriptions))
    .bind(&user.subscribed)
    .fetch_one(conn)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        rating: row.get("rating"),
        telegram_id: row.get("telegram_id"),
        instagram_link: row.get("instagram_link"),
        image_url: row.get("image_url"),
        subscriptions: row.get::<Json<Vec<Subscription>>, _>("subscriptions").0,
        subscribed: row.get("subscribed"),
    })
}

pub async fn delete(conn: &mut PgConnection, id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
} 