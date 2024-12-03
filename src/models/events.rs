use chrono::{NaiveDate, NaiveDateTime};
use rocket::serde::Deserialize;

use diesel::Queryable;
use serde::Serialize;
use rocket::form::FromForm;

use crate::database::events::EventType;

#[derive(Queryable, Serialize)]
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
    pub eventticketprice: i32,
    pub eventliked: Option<bool>,
}


#[derive(FromForm, Deserialize, Debug)]
pub struct EventFiltering {
    pub id: Option<i32>,
    pub userid: Option<i32>,
    pub eventname: Option<String>,
    pub eventdate: Option<String>,
    pub eventtype: Option<EventType>,
    pub eventcountry: Option<String>,
    pub eventcity: Option<String>,
    pub eventplace: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct Profile {
    id: i32,
    likes: i32,
}

