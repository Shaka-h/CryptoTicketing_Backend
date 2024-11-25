use chrono::{NaiveDate, NaiveDateTime};
use rocket::serde::Deserialize;

use diesel::Queryable;
use serde::Serialize;
use rocket::form::FromForm;

use crate::database::events::EventType;

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub userid: i32,
    pub eventname: String,
    pub eventdescription: String,
    pub eventdate: NaiveDate,
    pub eventdatetime: NaiveDateTime,
    pub eventtype: EventType,
    pub eventcountry: String,
    pub eventcity: String,
    pub eventplace: String,
    pub eventimage: String,
}


#[derive(FromForm, Deserialize, Debug)]
pub struct EventFiltering {
    pub id: i32,
    pub userid: i32,
    pub eventname: String,
    pub eventdate: String,
    pub eventdatetime: String,
    // pub eventtype: EventType,
    pub eventcountry: String,
    pub eventcity: String,
    pub eventplace: String,
}

#[derive(Serialize)]
pub struct Profile {
    id: i32,
    likes: i32,
}
