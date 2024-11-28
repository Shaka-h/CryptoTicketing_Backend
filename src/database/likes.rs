use crate::models::likes::{Like, LikesFiltering};
use crate::schema::likes;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

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
) -> Result<Vec<Like>, diesel::result::Error> {
    use crate::schema::likes::dsl::*;

    println!("filtlers {:?}", filters);
    if let Some(f) = filters {
        let mut query = likes.into_boxed();
        if let Some(id_filter) = f.id {
            query = query.filter(user_id.eq(id_filter));
        }
        if let Some(eventid_filter) = f.eventid {
            query = query.filter(event_id.eq(eventid_filter));
        }
        if let Some(limit_filter) = f.limit {
            query = query.limit(limit_filter);
        }

        query.load::<Like>(conn)
    } else {
        let results = likes
            .limit(5)
            .load::<Like>(conn)
            .expect("Error loading likes");

        Ok(results)
    }
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
