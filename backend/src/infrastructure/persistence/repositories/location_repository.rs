use sqlx::{types::uuid::Uuid, PgConnection, Row, types::Json};
use crate::domain::Location;
use crate::domain::models::location::Coordinates;

pub async fn create(conn: &mut PgConnection, location: &Location) -> Result<Location, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO locations (id, name, coordinates, confirmed, image_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, coordinates, confirmed, image_url
        "#
    )
    .bind(&location.id)
    .bind(&location.name)
    .bind(Json(&location.coordinates))
    .bind(&location.confirmed)
    .bind(&location.image_url)
    .fetch_one(conn)
    .await?;

    Ok(Location {
        id: row.get("id"),
        name: row.get("name"),
        coordinates: row.get::<Json<Coordinates>, _>("coordinates").0,
        confirmed: row.get("confirmed"),
        image_url: row.get("image_url"),
    })
}

pub async fn find_by_id(conn: &mut PgConnection, id: &Uuid) -> Result<Option<Location>, sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT id, name, coordinates, confirmed, image_url FROM locations WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;

    Ok(row.map(|row| Location {
        id: row.get("id"),
        name: row.get("name"),
        coordinates: row.get::<Json<Coordinates>, _>("coordinates").0,
        confirmed: row.get("confirmed"),
        image_url: row.get("image_url"),
    }))
}

pub async fn find_all_confirmed(conn: &mut PgConnection) -> Result<Vec<Location>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, name, coordinates, confirmed, image_url FROM locations WHERE confirmed = true"#
    )
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Location {
        id: row.get("id"),
        name: row.get("name"),
        coordinates: row.get::<Json<Coordinates>, _>("coordinates").0,
        confirmed: row.get("confirmed"),
        image_url: row.get("image_url"),
    }).collect())
}

pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<Location>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, name, coordinates, confirmed, image_url FROM locations"#
    )
    .fetch_all(conn)
    .await?;

    Ok(rows.into_iter().map(|row| Location {
        id: row.get("id"),
        name: row.get("name"),
        coordinates: row.get::<Json<Coordinates>, _>("coordinates").0,
        confirmed: row.get("confirmed"),
        image_url: row.get("image_url"),
    }).collect())
}

pub async fn update(conn: &mut PgConnection, id: &Uuid, location: &Location) -> Result<Location, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE locations
        SET name = $2, coordinates = $3, confirmed = $4, image_url = $5
        WHERE id = $1
        RETURNING id, name, coordinates, confirmed, image_url
        "#
    )
    .bind(id)
    .bind(&location.name)
    .bind(Json(&location.coordinates))
    .bind(&location.confirmed)
    .bind(&location.image_url)
    .fetch_one(conn)
    .await?;

    Ok(Location {
        id: row.get("id"),
        name: row.get("name"),
        coordinates: row.get::<Json<Coordinates>, _>("coordinates").0,
        confirmed: row.get("confirmed"),
        image_url: row.get("image_url"),
    })
}

pub async fn delete(conn: &mut PgConnection, id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM locations WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
} 