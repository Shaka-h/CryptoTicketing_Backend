#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket_sync_db_pools;

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

extern crate diesel;

#[macro_use]
extern crate validator_derive;

use dotenvy::dotenv;

mod auth;
mod config;
mod database;
mod errors;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::users::add_user,
                routes::users::get_users,
                routes::users::update_user,
                routes::users::delete_user,
                routes::events::add_event,
                routes::events::get_events,
                routes::events::update_event,
                routes::events::delete_event,
                routes::likes::like_event,
                routes::likes::delete_like,
                routes::likes::get_likes,
                routes::likes::is_event_liked_by_user
            ],
        )
        .attach(database::Db::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
