#[macro_use]
extern crate rocket;

mod domain;
mod infrastructure;
mod presentation;
mod application;

use rocket_db_pools::{Database};
use presentation::api::{
    users::{get_me, update_me},
    locations::{get_locations, get_all_locations},
    events::{get_events_by_location, get_upcoming_events, join_event, leave_event},
    auth::{login, register},
    admin::{
        get_users, set_user_role,
        create_location, update_location, delete_location, confirm_location,
        create_event, update_event, delete_event, confirm_event,
        create_event_timer, update_event_timer, delete_event_timer,
        import_users_csv, import_users_json
    },
    files::upload
};
use presentation::middleware::cors::CORS;

#[derive(Database)]
#[database("postgres")]
pub struct AppDatabase(rocket_db_pools::sqlx::PgPool);

#[options("/<_..>")]
fn options() -> &'static str {
    ""
}

#[launch]
fn rocket() -> _ {
    use tracing_subscriber::{Registry, prelude::*};
    use tracing_subscriber::fmt::{self};

    let subscriber = Registry::default()
        .with(fmt::layer().event_format(fmt::format::Format::default().json()));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default subscriber");

    rocket::build()
        .attach(AppDatabase::init())
        .attach(CORS)
        .mount("/api/auth", routes![login, register])
        .mount("/api", routes![
            login, register,
            get_me, update_me,
            get_locations, get_all_locations,
            get_events_by_location, get_upcoming_events, join_event, leave_event,
            upload
        ])
        .mount("/api/admin", routes![
            get_users, set_user_role,
            create_location, update_location, delete_location, confirm_location,
            create_event, update_event, delete_event, confirm_event,
            create_event_timer, update_event_timer, delete_event_timer,
            import_users_csv, import_users_json
        ])
}
