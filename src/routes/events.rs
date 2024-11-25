// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{r2d2::event, Queryable};
use rocket::serde::{
    Deserialize, Serialize,
    json::{json, Json, Value},
};
use crate::database::{self, events::EventType, users::UserCreationError, Db};
use crate::errors::{Errors, FieldValidator};
use crate::{
    config::AppState,
    models::user::UserFiltering,
};
use rocket::State;

#[derive(Queryable)]
struct NewEventData {
    id: Option<i32>,
    userid: Option<i32>,
    eventname: Option<String>,
    eventdescription: Option<String>,
    eventdate: NaiveDate,
    eventdatetime: NaiveDateTime,
    eventtype: EventType,
    eventcountry: Option<String>,
    eventcity: Option<String>,
    eventplace: Option<String>,
    eventimage: Option<String>,
}

#[derive(Queryable, Deserialize)]
pub struct NewEvent {
    event: NewEventData,
}

#[post("/event", format = "json", data = "<new_event>")]
pub async fn add_event(
    new_event: Json<NewEvent>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {

    db.run(move |conn| {
        database::events::create(
            conn, 
            new_event.userid,
            new_event.eventname,
            new_event.eventdescription,
            new_event.eventdate,
            new_event.eventdatetime,
            new_event.eventtype,
            new_event.eventcountry,
            new_event.eventcity,
            new_event.eventplace,
            new_event.eventimage,
        )
            .map(|event| json!({ "event": event }))
            .map_err(|_| Errors::new(&[("database", "failed to create event")]))
    })
    .await
}

// #[get("/get_events?<username>&<user_id>&<email>")]
// pub async fn get_events(
//     db: Db,
//     user_id: i32,
//     username: String,
//     email: String,
// ) -> Result<Value, Errors> {
//     // let filters = filters.map(|f| f.into_inner());
//     db.run(move |conn| {
//         database::events::get_events(
//             conn,
//             Some(UserFiltering {
//                 id: user_id,
//                 username,
//                 email,
//                 limit: 100,
//             }),
//         )
//         .map(|events| json!({ "events": events }))
//         .map_err(|_| Errors::new(&[("database", "failed to fetch events")]))
//     })
//     .await

//     // Ok(format!("events"))
// }

// #[derive(Deserialize)]
// pub struct UpdateUser {
//     id: i32,
//     user: database::events::UpdateUserData,
// }

// #[put("/user", format = "json", data = "<user>")]
// pub async fn update_user(user: Json<UpdateUser>, db: Db) -> Option<Value> {
//     db.run(move |conn| database::users::update(conn, user.id, &user.user))
//         .await
//         .map(|user| json!({ "user": user }))
// }

// #[derive(Deserialize)]
// pub struct DeleteUser {
//     id: i32,
// }

// #[delete("/user", format = "json", data = "<user>")]
// pub async fn delete_user(user: Json<DeleteUser>, db: Db) -> Result<Value, Errors> {
//     db.run(move |conn| database::users::delete(conn, user.id))
//         .await
//         .map(|_| json!({ "message": "User deleted successfully" }))
//         .map_err(|_| Errors::new(&[("database", "failed to delete user")]))
// }
