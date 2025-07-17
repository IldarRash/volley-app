#[macro_use] extern crate rocket;

mod api;
mod models;
mod persistence;
mod cors;

use api::beosand::{get_locations, get_events_by_location};
use api::auth::{register, login, AuthUser};
use persistence::db::get_db;
use crate::models::location::Location;
use crate::persistence::repository::location_repository;
use rocket::fairing::AdHoc;
use mongodb::bson::doc;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn seed_data() {
    println!("Attempting to get database connection...");
    let db = get_db().await.unwrap();
    println!("Database connection successful.");

    println!("Clearing existing locations...");
    location_repository::clear(&db).await.ok();

    println!("Seeding database with initial data...");
    let ada = Location {
        name: Some("Ada Ciganlija".to_string()),
        coordinates: [44.788, 20.415],
        confirmed: true,
    };
    location_repository::create(&db, &ada).await.ok();
    println!("Database seeded.");
}

#[get("/protected")]
fn protected(user: AuthUser) -> String {
    format!("Hello, {}! You are an authenticated user.", user.0.sub)
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8080));

    rocket::custom(figment)
        .attach(cors::CORS)
        .attach(AdHoc::on_liftoff("Seed Data", |_| Box::pin(async move {
            seed_data().await;
        })))
        .mount("/", routes![index, get_locations, get_events_by_location, protected])
        .mount("/auth", routes![register, login])
}
