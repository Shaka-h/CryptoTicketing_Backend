// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::database::{self,Db};
use crate::errors::Errors;
use crate::models::likes::LikesFiltering;
use diesel::Queryable;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize,
};
#[derive(Queryable, Deserialize)]
struct NewLikeData {
    id: Option<i32>,
    userid: i32,
    eventid: i32,
}

#[derive(Deserialize)]
pub struct NewLike {
    like: NewLikeData,
}

#[post("/like", format = "json", data = "<new_like>")]
pub async fn like_event(
    new_like: Json<NewLike>,
    db: Db,
) -> Result<Value, Errors> {
    let new_like = new_like.into_inner().like;

    db.run(move |conn| {
        database::likes::create(
            conn,
            new_like.userid,
            new_like.eventid,
        )
        .map(|like| json!({ "like": like }))
        .map_err(|_| Errors::new(&[("database", "failed to like an event")]))
    })
    .await
}

#[get("/get_likes?<filters..>")]
pub async fn get_likes(
    db: Db,
    filters: Option<LikesFiltering>
) -> Result<Value, Errors> {
    db.run(move |conn| {
        database::likes::get_likes(
            conn,
            filters,
        )
        .map(|events| json!({ "events": events }))
        .map_err(|_| Errors::new(&[("database", "failed to fetch events")]))
    })
    .await

}


#[derive(Deserialize)]
pub struct DeleteLike {
    id: i32,
}


#[delete("/like", format = "json", data = "<like>")]
pub async fn delete_like(like: Json<DeleteLike>, db: Db) -> Result<Value, Errors> {
    db.run(move |conn| database::likes::delete(conn, like.id))
        .await
        .map(|_| json!({ "message": "like deleted successfully" }))
        .map_err(|_| Errors::new(&[("database", "failed to delete like")]))
}
