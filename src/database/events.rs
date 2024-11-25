use crate::models::events::{Event, EventFiltering};
use crate::schema::events;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::PgConnection;
use diesel::{prelude::*, serialize};
use diesel::result::{DatabaseErrorKind, Error};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum EventType {
    Music,
    Games,
    Performing,
    Movies,
    Tour,
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
}

pub enum EventCreationError {
    NonExistUsername,
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
) -> Result<Event,  diesel::result::Error> {
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
    };

    diesel::insert_into(events::table)
        .values(new_event)
        .get_result::<Event>(conn)
        .map_err(Into::into)
}

// pub fn get_users(
//     conn: &mut PgConnection,
//     filters: Option<EventFiltering>,
// ) -> Result<Vec<Event>, diesel::result::Error> {
//     use crate::schema::events::dsl::*;

//     // let mut query = users.into_boxed();
//     println!("filtlers {:?}", filters);

//     if let Some(f) = filters {
//         let mut query = events.into_boxed();

//         query
//             .limit(5) // Limit to 5 results
//             .load::<Event>(conn) // Load results into Vec<User>
//     } else {
//         let results = events
//             .limit(5)
//             // .select(User::as_select())
//             .load::<Event>(conn) // Load results into Vec<User>
//             .expect("Error loading users");
//         Ok(results)
//     }
// }

// // TODO: remove clone when diesel will allow skipping fields
// #[derive(Deserialize, AsChangeset, Default, Clone, Validate)]
// #[table_name = "events"]
// pub struct UpdateEventData {
//     username: Option<String>,
//     email: Option<String>,
//     image: Option<String>,
// }

// pub fn update(conn: &mut PgConnection, id: i32, data: &UpdateEventData) -> Option<Event> {
//     let new_event_data = &UpdateEventData { ..data.clone() };
//     diesel::update(events::table.find(id))
//         .set(new_event_data)
//         .get_result(conn)
//         .ok()
// }

pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, Error> {
    let event_deleted = diesel::delete(events::table.filter(events::id.eq(id)))
        .execute(conn)
        .expect("Error deleting user");

    println!("Deleted {} user", event_deleted);

    Ok(event_deleted)
}
