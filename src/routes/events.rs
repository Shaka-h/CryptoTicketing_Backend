// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::database::{self, events::EventType, users::UserCreationError, Db};
use crate::errors::{Errors, FieldValidator};
use crate::models::events::EventFiltering;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{r2d2::event, Queryable};
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize,
};

#[derive(Queryable, Deserialize)]
struct NewEventData {
    id: Option<i32>,
    userid: i32,
    eventname: String,
    eventdescription: String,
    eventdate: String,
    eventdatetime: String,
    eventtype: EventType,
    eventcountry: String,
    eventcity: String,
    eventplace: String,
    eventimage: String,
    eventticketprice: i32,
    eventliked: bool,
}

#[derive(Deserialize)]
pub struct NewEvent {
    event: NewEventData,
}

#[post("/event", format = "json", data = "<new_event>")]
pub async fn add_event(new_event: Json<NewEvent>, db: Db) -> Result<Value, Errors> {
    let new_event = new_event.into_inner().event;
    // Parse `eventdate` into NaiveDate
    let parsed_eventdate = NaiveDate::parse_from_str(&new_event.eventdate, "%Y-%m-%d")
        .map_err(|_| Errors::new(&[("eventdate", "invalid date format, expected YYYY-MM-DD")]))?;

    // Parse `eventdatetime` into NaiveDateTime
    let parsed_eventdatetime =
        NaiveDateTime::parse_from_str(&new_event.eventdatetime, "%Y-%m-%d %H:%M:%S").map_err(
            |_| {
                Errors::new(&[(
                    "eventdatetime",
                    "invalid datetime format, expected YYYY-MM-DD HH:MM:SS",
                )])
            },
        )?;

    db.run(move |conn| {
        database::events::create(
            conn,
            new_event.userid,
            &new_event.eventname,
            &new_event.eventdescription,
            parsed_eventdate,
            parsed_eventdatetime,
            new_event.eventtype,
            &new_event.eventcountry,
            &new_event.eventcity,
            &new_event.eventplace,
            &new_event.eventimage,
            new_event.eventticketprice,
        )
        .map(|event| json!({ "event": event }))
        .map_err(|_| Errors::new(&[("database", "failed to create event")]))
    })
    .await
}

#[get("/get_events?<filters..>")]
pub async fn get_events(db: Db, filters: Option<EventFiltering>) -> Result<Value, Errors> {
    // let filters = filters.map(|f| f.into_inner());
    db.run(move |conn| {
        database::events::get_events(conn, filters)
            .map(|events| json!({ "events": events }))
            .map_err(|_| Errors::new(&[("database", "failed to fetch events")]))
    })
    .await

    // Ok(format!("events"))
}

#[derive(Deserialize)]
pub struct UpdateEvent {
    id: i32,
    event: database::events::UpdateEventData,
}

#[put("/event", format = "json", data = "<event>")]
pub async fn update_event(event: Json<UpdateEvent>, db: Db) -> Option<Value> {
    db.run(move |conn| database::events::update(conn, event.id, &event.event))
        .await
        .map(|event| json!({ "event": event }))
}

#[derive(Deserialize)]
pub struct DeleteEvent {
    id: i32,
}

#[delete("/event", format = "json", data = "<event>")]
pub async fn delete_event(event: Json<DeleteEvent>, db: Db) -> Result<Value, Errors> {
    db.run(move |conn| database::events::delete(conn, event.id))
        .await
        .map(|_| json!({ "message": "event deleted successfully" }))
        .map_err(|_| Errors::new(&[("database", "failed to delete event")]))
}
