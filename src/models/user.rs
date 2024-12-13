use crate::auth::Auth;
use chrono::{Duration, Utc};
use rocket::serde::Deserialize;

use diesel::Queryable;
use serde::Serialize;
type Url = String;
use rocket::form::FromForm;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub image: Option<Url>,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(FromForm, Deserialize, Debug)]
pub struct UserFiltering {
    pub id: Option<i32>,        
    pub username: Option<String>, 
    pub email: Option<String>,   
    pub limit: Option<i64>,      
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    id: &'a i32,
    username: &'a str,
    email: &'a str,
    image: Option<&'a str>,
    token: String,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    image: Option<String>,
    following: bool,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            id: &self.id,
            username: &self.username,
            email: &self.email,
            image: self.image.as_ref().map(String::as_str),
            token,
        }
    }

    pub fn to_profile(self, following: bool) -> Profile {
        Profile {
            username: self.username,
            image: self.image,
            following,
        }
    }
}
