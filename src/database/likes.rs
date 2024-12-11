use crate::database::events::EventResult;
use crate::models::events::Event;
use crate::schema::events;
use crate::models::likes::{Like, LikesFiltering};
use crate::routes::likes::LikeRequest;
use crate::schema::likes;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::dsl::exists;
use diesel::dsl::select;
use rocket::serde::json::Json;

use super::events::EventsLogged;

#[derive(Insertable, Queryable, QueryableByName)]
#[table_name = "likes"]
pub struct NewLike {
    pub user_id: i32,
    pub event_id: i32,
}

pub enum LikeCreationError {
    NonExistUserId,
    NonExistEventId,
    DuplicatedUserId,
    Other,
}

impl From<Error> for LikeCreationError {
    fn from(err: Error) -> LikeCreationError {
        if let Error::DatabaseError(kind, info) = &err {
            match kind {
                DatabaseErrorKind::ForeignKeyViolation => {
                    if let Some(constraint) = info.constraint_name() {
                        match constraint {
                            "likes_user_id_fkey" => return LikeCreationError::NonExistUserId,
                            "likes_event_id_fkey" => return LikeCreationError::NonExistEventId,
                            _ => {}
                        }
                    }
                }
                DatabaseErrorKind::UniqueViolation => {
                    if let Some(constraint) = info.constraint_name() {
                        if constraint == "users_id_key" {
                            return LikeCreationError::DuplicatedUserId;
                        }
                    }
                }
                _ => {}
            }
        }
        LikeCreationError::Other
    }
}

pub fn create(
    conn: &mut PgConnection,
    user_id: i32,
    event_id: i32,
) -> Result<Like, LikeCreationError> {
    let new_like = &NewLike { user_id, event_id };

    diesel::insert_into(likes::table)
        .values(new_like)
        .get_result::<Like>(conn)
        .map_err(Into::into)
}

pub fn delete(conn: &mut PgConnection, user_id: i32) -> Result<usize, Error> {
    let like_deleted = diesel::delete(likes::table.filter(likes::user_id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting user");

    println!("Deleted {} user", like_deleted);

    Ok(like_deleted)
}

pub fn get_likes(
    conn: &mut PgConnection,
    filters: Option<LikesFiltering>,
) -> Result<Vec<EventsLogged>, diesel::result::Error> {
    use crate::schema::likes::dsl::*;
    use crate::schema::events::dsl::*;

    println!("filtlers {:?}", filters);
    let mut query = likes.into_boxed();
        if let Some(id_filter) = filters.unwrap().id {
            query = query.filter(user_id.eq(id_filter));
        }
        // if let Some(eventid_filter) = filters.unwrap().eventid {
        //     query = query.filter(event_id.eq(eventid_filter));
        // }
        // if let Some(limit_filter) = filters.unwrap().limit {
        //     query = query.limit(limit_filter);
        // }

        let likes_result = query.load::<Like>(conn)?;

        let event_like_object = likes_result.into_iter().map(|event_like| {

            let event = events
                .filter(id.eq(event_like.event_id)) // Ensure to filter by event's id
                .first::<Event>(conn)
                .unwrap();

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
                eventliked: true,
            })
        }).collect::<Result<Vec<EventsLogged>, diesel::result::Error>>()?;

        Ok(event_like_object)
}



pub fn is_event_liked_by_user(
    conn: &mut PgConnection,
    data: Json<LikeRequest>
) -> QueryResult<bool> {
    use crate::schema::likes::dsl::*;
    // Build the EXISTS query
    let is_liked_query = select(exists(
        likes
            .filter(user_id.eq(data.user_id))
            .filter(event_id.eq(data.event_id)),
    ));

    // Execute the query and return the result
    is_liked_query.get_result(conn)
}


// pub fn get_popular_likes(conn: &mut PgConnection) -> Result<Vec<(Like)>, Error> {
//     use crate::schema::likes::dsl::*;
//     use diesel::dsl::count;

//     let result = likes
//         .filter(event_id.is_not_null())
//         .select((count(event_id)))
//         .group_by(event_id)
//         .order_by(count(event_id).desc())
//         .limit(20)
//         .load::<(Like)>(conn)
//         .expect("Error loading likes");

//     Ok(result)
// }

// pub fn get_popular_likes(conn: &mut PgConnection) -> Result<Vec<(i32, i64)>, diesel::result::Error> {
//     use crate::schema::likes::dsl::*;
//     use diesel::dsl::count;
//     use diesel::sql::functions::Join;

//     let subquery = likes
//         .filter(event_id.is_not_null())
//         .select((count(event_id)))
//         .group_by(event_id)
//         .as("subquery");

//     likes
//         .inner_join(subquery)
//         .on(likes.event_id.eq(subquery.column::<i32>("event_id")))
//         .select((likes::user_id, subquery.column::<i64>("count")))
//         .order_by(subquery.column::<i64>("count").desc())
//         .limit(20)
//         .load::<(i32, i64)>(conn)
// }

// pub fn get_popular_likes(
//     conn: &mut PgConnection,
//     filters: Option<LikesFiltering>,
// ) -> Result<Vec<Like>, diesel::result::Error> {
//     use crate::schema::likes::dsl::*;

//     println!("filtlers {:?}", filters);

//     if let Some(f) = filters {
//         let mut query = likes.into_boxed();
//         if let Some(id_filter) = f.id {
//             query = query.filter(user_id.eq(id_filter));
//         }
//         if let Some(eventid_filter) = f.eventid {
//             query = query.filter(event_id.eq(eventid_filter));
//         }
//         if let Some(limit_filter) = f.limit {
//             query = query.limit(limit_filter);
//         }

//         query.limit(5).load::<Like>(conn)
//     } else {
//         let results = likes
//             .filter(event_id.is_not_null())
//             .select((event_id, count(event_id)))
//             .group_by(event_id)
//             .order(count(event_id).desc())
//             .limit(20)
//             .load::<Like>(conn)
//             .expect("Error loading popular likes");

//         Ok(results)
//     }
// }
