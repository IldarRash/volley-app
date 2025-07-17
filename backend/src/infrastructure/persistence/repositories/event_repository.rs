use sqlx::{types::uuid::Uuid, PgConnection, types::chrono::Utc, types::Json, Row};
use crate::domain::{Event, Participant, ParticipantStatus, PaymentStatus, EventType};

pub async fn create(conn: &mut PgConnection, event: &Event) -> Result<Event, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO events (id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price, trainer_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price, trainer_id
        "#
    )
    .bind(&event.id)
    .bind(&event.name)
    .bind(&event.description)
    .bind(&event.datetime)
    .bind(&event.location_id)
    .bind(&event.event_type)
    .bind(&event.max_participants)
    .bind(Json(&event.participants))
    .bind(&event.level)
    .bind(&event.confirmed)
    .bind(&event.price)
    .bind(&event.trainer_id)
    .fetch_one(conn)
    .await?;

    Ok(Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
        trainer_id: row.get("trainer_id"),
    })
}

pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<Event>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price FROM events"#
    )
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    }).collect())
}

pub async fn find_by_location_id(conn: &mut PgConnection, location_id: &Uuid) -> Result<Vec<Event>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price FROM events WHERE location_id = $1"#
    )
    .bind(location_id)
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    }).collect())
}

pub async fn find_by_id(conn: &mut PgConnection, id: &Uuid) -> Result<Option<Event>, sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price FROM events WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;

    Ok(row.map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    }))
}

pub async fn find_by_user(conn: &mut PgConnection, user_id: &Uuid) -> Result<Vec<Event>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price
        FROM events, jsonb_to_recordset(participants) as p(user_id uuid)
        WHERE p.user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    }).collect())
}

pub async fn find_upcoming(conn: &mut PgConnection, limit: i64) -> Result<Vec<Event>, sqlx::Error> {
    let now = Utc::now();
    let rows = sqlx::query(
        r#"
        SELECT id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price 
        FROM events
        WHERE datetime >= $1 AND confirmed = true
        ORDER BY datetime
        LIMIT $2
        "#
    )
    .bind(now)
    .bind(limit)
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    }).collect())
}

pub async fn update(conn: &mut PgConnection, id: &Uuid, event: &Event) -> Result<Event, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE events
        SET name = $2, description = $3, datetime = $4, location_id = $5, event_type = $6, max_participants = $7, participants = $8, level = $9, confirmed = $10, price = $11
        WHERE id = $1
        RETURNING id, name, description, datetime, location_id, event_type, max_participants, participants, level, confirmed, price
        "#
    )
    .bind(id)
    .bind(&event.name)
    .bind(&event.description)
    .bind(&event.datetime)
    .bind(&event.location_id)
    .bind(&event.event_type)
    .bind(&event.max_participants)
    .bind(Json(&event.participants))
    .bind(&event.level)
    .bind(&event.confirmed)
    .bind(&event.price)
    .fetch_one(conn)
    .await?;

    Ok(Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        datetime: row.get("datetime"),
        location_id: row.get("location_id"),
        event_type: row.get("event_type"),
        max_participants: row.get("max_participants"),
        participants: row.get::<Json<Vec<Participant>>, _>("participants").0,
        level: row.get("level"),
        confirmed: row.get("confirmed"),
        price: row.get("price"),
    })
}

pub async fn add_participant(conn: &mut PgConnection, event_id: &Uuid, participant: &Participant) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE events SET participants = participants || $1::jsonb WHERE id = $2"
    )
    .bind(Json(participant))
    .bind(event_id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn update_participant_status(
    conn: &mut PgConnection,
    event_id: &Uuid,
    user_id: &Uuid,
    status: &ParticipantStatus,
    payment: &PaymentStatus,
) -> Result<(), sqlx::Error> {
    let status_str = serde_json::to_string(status).unwrap();
    let payment_str = serde_json::to_string(payment).unwrap();

    sqlx::query(
        r#"
        UPDATE events
        SET participants = (
            SELECT jsonb_agg(
                CASE
                    WHEN (p->>'user_id')::uuid = $2 THEN
                        jsonb_set(jsonb_set(p, '{status}', $3::jsonb), '{payment}', $4::jsonb)
                    ELSE p
                END
            )
            FROM jsonb_array_elements(participants) p
        )
        WHERE id = $1
        "#
    )
    .bind(event_id)
    .bind(user_id)
    .bind(&status_str)
    .bind(&payment_str)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn remove_participant(conn: &mut PgConnection, event_id: &Uuid, user_id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE events
        SET participants = (
            SELECT jsonb_agg(p)
            FROM jsonb_array_elements(participants) p
            WHERE (p->>'user_id')::uuid != $1
        )
        WHERE id = $2
        "#
    )
    .bind(user_id)
    .bind(event_id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn delete(conn: &mut PgConnection, id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
} 