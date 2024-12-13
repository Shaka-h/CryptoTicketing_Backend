use crate::models::events::{Event, EventFiltering};
use crate::schema::events;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::expression::AsExpression;
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{prelude::*, serialize};
use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    Music,
    Games,
    Performing,
    Movies,
    Tour,
}

impl FromStr for EventType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "music" => Ok(EventType::Music),
            "games" => Ok(EventType::Games),
            "performing" => Ok(EventType::Performing),
            "movies" => Ok(EventType::Movies),
            "tour" => Ok(EventType::Tour),
            _ => Err(()),
        }
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for EventType {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        field.value.parse::<Self>().map_err(|_| {
            rocket::form::Errors::from(rocket::form::Error::validation("invalid event type"))
        })
    }
}

impl ToSql<Text, diesel::pg::Pg> for EventType {
    fn to_sql(&self, out: &mut Output<diesel::pg::Pg>) -> serialize::Result {
        let value = match self {
            EventType::Music => "Music",
            EventType::Games => "Games",
            EventType::Performing => "Performing",
            EventType::Movies => "Movies",
            EventType::Tour => "Tour",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::pg::Pg> for EventType {
    fn from_sql(bytes: diesel::backend::RawValue<diesel::pg::Pg>) -> deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes())? {
            "Music" => Ok(EventType::Music),
            "Games" => Ok(EventType::Games),
            "Performing" => Ok(EventType::Performing),
            "Movies" => Ok(EventType::Movies),
            "Tour" => Ok(EventType::Tour),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub userid: i32,
    pub eventname: &'a str,
    pub eventdescription: &'a str,
    pub eventdate: NaiveDate,
    pub eventdatetime: NaiveDateTime,
    pub eventtype: &'a EventType,
    pub eventcountry: &'a str,
    pub eventcity: &'a str,
    pub eventplace: &'a str,
    pub eventimage: &'a str,
    pub eventticketprice: i32,
}

#[derive(Serialize, Debug)]
pub struct EventsLogged {
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
    pub eventliked: bool,
}

pub enum EventCreationError {
    NonExistUsername,
}

#[derive(Serialize)]
pub enum EventResult {
    UserLogged(Vec<EventsLogged>),
    UnLoggedUser(Vec<Event>),
}

pub fn create(
    conn: &mut PgConnection,
    userid: i32,
    eventname: &str,
    eventdescription: &str,
    eventdate: NaiveDate,
    eventdatetime: NaiveDateTime,
    eventtype: EventType,
    eventcountry: &str,
    eventcity: &str,
    eventplace: &str,
    eventimage: &str,
    eventticket_price: i32,
) -> Result<Event, diesel::result::Error> {
    let new_event = &NewEvent {
        userid,
        eventname,
        eventdescription,
        eventdate,
        eventdatetime,
        eventtype: &eventtype,
        eventcountry,
        eventcity,
        eventplace,
        eventimage,
        eventticketprice: eventticket_price,
    };

    diesel::insert_into(events::table)
        .values(new_event)
        .get_result::<Event>(conn)
        .map_err(Into::into)
}

pub fn get_events(
    conn: &mut PgConnection,
    filters: Option<EventFiltering>,
) -> Result<Vec<EventResult>, diesel::result::Error> {
    use crate::schema::events::dsl::*;
    use crate::schema::likes::dsl::*;

    if let Some(f) = filters {
        let mut query = events.into_boxed();
        if let Some(ref eventname_filter) = f.eventname {
            query = query.filter(eventname.eq(eventname_filter));
        }
        if let Some(id_filter) = f.id {
            query = query.filter(id.eq(id_filter));
        }
        if let Some(userid_filter) = f.userid {
            query = query.filter(userid.eq(userid_filter));
        }
        if let Some(limit_filter) = f.limit {
            query = query.limit(limit_filter);
        }
        if let Some(ref eventcountry_filter) = f.eventcountry {
            query = query.filter(eventcountry.eq(eventcountry_filter));
        }
        if let Some(ref eventcity_filter) = f.eventcity {
            query = query.filter(eventcity.eq(eventcity_filter));
        }
        if let Some(ref eventplace_filter) = f.eventplace {
            query = query.filter(eventplace.eq(eventplace_filter));
        }
        if let Some(ref eventdate_filter) = f.eventdate {
            if let Ok(parsed_date) = NaiveDate::parse_from_str(eventdate_filter, "%Y-%m-%d") {
                query = query.filter(eventdate.eq(parsed_date));
            } else {
                eprintln!("Invalid date format for eventdate_filter");
            }
        }
        if let Some(ref eventtype_filter) = f.eventtype {
            query = query.filter(eventtype.eq(eventtype_filter));
        }

        let result = query.limit(5).load::<Event>(conn)?;

        let events_logged: Vec<EventsLogged> = result
            .into_iter()
            .map(|event| {
                let is_liked_query = select(exists(
                    likes
                        .filter(user_id.eq(event.userid))
                        .filter(event_id.eq(event.id)),
                ));

                let is_event_liked = is_liked_query.get_result(conn)?;

                Ok(EventsLogged {
                    id: event.id,
                    userid: event.userid,
                    eventname: event.eventname,
                    eventdescription: event.eventdescription,
                    eventdate: event.eventdate,
                    eventdatetime: event.eventdatetime,
                    eventcountry: event.eventcountry,
                    eventcity: event.eventcity,
                    eventplace: event.eventplace,
                    eventimage: event.eventimage,
                    eventtype: event.eventtype,
                    eventticketprice: event.eventticketprice,
                    eventliked: is_event_liked,
                })
            })
            .collect::<Result<Vec<EventsLogged>, diesel::result::Error>>()?;

        // let mut events_logged: Vec<EventsLogged> = Vec::new();


        // for event in result.into_iter() {
        //     let is_liked_query = select(exists(
        //         likes
        //             .filter(user_id.eq(event.userid))
        //             .filter(event_id.eq(event.id)),
        //     ));


        //     let is_event_liked = is_liked_query.get_result(conn)?;

        //     events_logged.push(EventsLogged {
        //         userid: event.userid,
        //         eventname: event.eventname,
        //         eventdescription: event.eventdescription,
        //         eventdate: event.eventdate,
        //         eventdatetime: event.eventdatetime,
        //         eventcountry: event.eventcountry,
        //         eventcity: event.eventcity,
        //         eventplace: event.eventplace,
        //         eventimage: event.eventimage,
        //         eventtype: event.eventtype,
        //         eventticketprice: event.eventticketprice,
        //         eventliked: is_event_liked,
        //     });
        // }

        // println!("Logged events {:#?}", events_logged);

        // Ok(events_logged)
        Ok(vec![EventResult::UserLogged(events_logged)])
    } else {
        return Ok(vec![EventResult::UnLoggedUser(
            events.limit(5).load::<Event>(conn)?,
        )]);
    }
}

pub mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S"; // Expected format: "2024-12-25 19:30:00"

    // Serialize the NaiveDateTime into the expected format
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = date.format(FORMAT).to_string();
        serializer.serialize_str(&formatted)
    }

    // Deserialize the string into NaiveDateTime
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_str = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&date_str, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "events"]
pub struct UpdateEventData {
    userid: Option<i32>,
    eventname: Option<String>,
    eventdescription: Option<String>,
    eventdate: NaiveDate,
    // #[serde(deserialize_with = "date_format")]
    eventdatetime: NaiveDateTime,
    eventtype: Option<EventType>,
    eventcountry: Option<String>,
    eventcity: Option<String>,
    eventplace: Option<String>,
    eventimage: Option<String>,
}

pub fn update(conn: &mut PgConnection, id: i32, data: &UpdateEventData) -> Option<Event> {
    let new_event_data = &UpdateEventData { ..data.clone() };
    diesel::update(events::table.find(id))
        .set(new_event_data)
        .get_result(conn)
        .ok()
}

pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, Error> {
    let event_deleted = diesel::delete(events::table.filter(events::id.eq(id)))
        .execute(conn)
        .expect("Error deleting event");

    println!("Deleted {} event", event_deleted);

    Ok(event_deleted)
}
