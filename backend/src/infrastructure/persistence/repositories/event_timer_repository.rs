use sqlx::{types::uuid::Uuid, PgConnection};
use crate::domain::EventTimer;

pub async fn create(conn: &mut PgConnection, timer: &EventTimer) -> Result<EventTimer, sqlx::Error> {
    sqlx::query_as::<_, EventTimer>(
        "INSERT INTO event_timers (id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants"
    )
    .bind(timer.id)
    .bind(&timer.name)
    .bind(timer.location_id)
    .bind(timer.event_type.as_ref())
    .bind(timer.level.as_deref())
    .bind(timer.day_of_week)
    .bind(timer.time)
    .bind(timer.active)
    .bind(timer.price)
    .bind(timer.max_participants)
    .fetch_one(conn)
    .await
}

pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<EventTimer>, sqlx::Error> {
    sqlx::query_as::<_, EventTimer>(
        "SELECT id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants FROM event_timers"
    )
    .fetch_all(conn)
    .await
}

pub async fn find_active(conn: &mut PgConnection) -> Result<Vec<EventTimer>, sqlx::Error> {
    sqlx::query_as::<_, EventTimer>(
        "SELECT id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants FROM event_timers WHERE active = true"
    )
    .fetch_all(conn)
    .await
}

pub async fn find_by_id(conn: &mut PgConnection, id: &Uuid) -> Result<Option<EventTimer>, sqlx::Error> {
    sqlx::query_as::<_, EventTimer>(
        "SELECT id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants FROM event_timers WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(conn)
    .await
}

pub async fn update(conn: &mut PgConnection, id: &Uuid, timer: &EventTimer) -> Result<EventTimer, sqlx::Error> {
    sqlx::query_as::<_, EventTimer>(
        "UPDATE event_timers
        SET name = $2, location_id = $3, event_type = $4, level = $5, day_of_week = $6, time = $7, active = $8, price = $9, max_participants = $10
        WHERE id = $1
        RETURNING id, name, location_id, event_type, level, day_of_week, time, active, price, max_participants"
    )
    .bind(id)
    .bind(&timer.name)
    .bind(timer.location_id)
    .bind(timer.event_type.as_ref())
    .bind(timer.level.as_deref())
    .bind(timer.day_of_week)
    .bind(timer.time)
    .bind(timer.active)
    .bind(timer.price)
    .bind(timer.max_participants)
    .fetch_one(conn)
    .await
}

pub async fn toggle_active(conn: &mut PgConnection, id: &Uuid, active: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE event_timers SET active = $1 WHERE id = $2")
        .bind(active)
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete(conn: &mut PgConnection, id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM event_timers WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
} 