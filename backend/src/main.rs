#[macro_use]
extern crate rocket;

mod domain;
mod application;
mod infrastructure;
mod presentation;

use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket_db_pools::{Database, Connection};

use infrastructure::external::telegram;
use infrastructure::persistence::repositories::location_repository;
use presentation::api::{admin, auth, events, files, locations, users};
use presentation::middleware::cors::CORS;

use domain::Location;

#[derive(Database)]
#[database("mongodb_volley")]
pub struct Db(mongodb::Client);

#[get("/")]
fn index() -> &'static str {
    "Beosend Volleyball API"
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

async fn seed_data(db: mongodb::Database) {
    println!("Seeding data...");
    
    let location = Location::new(
        "Ada Ciganlija".to_string(),
        [44.788, 20.415]
    );

    if let Err(e) = location_repository::create(&db, &location).await {
        eprintln!("Failed to seed location: {}", e);
    }

    println!("Seeding complete.");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(CORS)
        .attach(AdHoc::on_liftoff("Seed Data", |rocket| {
            Box::pin(async move {
                let db_conn = Db::fetch(rocket).unwrap();
                let db = &db_conn.database("volleyApp");
                seed_data(db.clone()).await;
            })
        }))
        .attach(AdHoc::on_liftoff("Telegram Bot", |_| {
            Box::pin(async move {
                tokio::spawn(telegram::run());
            })
        }))
        .mount("/", routes![
            index,
            health
        ])
        .mount("/auth", routes![
            auth::register,
            auth::login,
        ])
        .mount("/admin", routes![
            admin::import_users_csv,
            admin::get_users,
            admin::set_user_role,
            admin::create_location,
            admin::update_location,
            admin::delete_location,
            admin::confirm_location,
            admin::create_event,
            admin::update_event,
            admin::delete_event,
            admin::confirm_event,
            admin::create_event_timer,
            admin::update_event_timer,
            admin::delete_event_timer,
        ])
        .mount("/users", routes![
            users::get_me,
            users::update_me,
        ])
        .mount("/files", routes![
            files::upload,
        ])
        .mount("/locations", routes![
            locations::get_locations,
            locations::get_all_locations,
        ])
        .mount("/events", routes![
            events::get_events_by_location,
            events::get_upcoming_events,
            events::join_event,
            events::leave_event,
        ])
        .mount("/static", FileServer::from("static"))
}
