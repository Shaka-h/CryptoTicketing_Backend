use rocket::serde::Deserialize;
use diesel::Queryable;
use serde::Serialize;
use rocket::form::FromForm;


#[derive(Queryable, Serialize)]
pub struct Like {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(FromForm, Deserialize, Debug)]
pub struct LikesFiltering {
    pub id: Option<i32>,
    pub userid: Option<i32>,
    pub eventid: Option<i32>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct Profile {
    id: i32,
    likes: i32,
}

