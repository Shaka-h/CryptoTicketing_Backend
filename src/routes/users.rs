// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
use rocket::serde::{
    Deserialize, Serialize,
    json::{json, Json, Value},
};
use crate::database::{self, users::UserCreationError, Db};
use crate::errors::{Errors, FieldValidator};
use crate::{auth::Auth, schema::users::id};
use crate::{
    config::AppState,
    database::users::UpdateUserData,
    models::user::{User, UserFiltering},
};
use rocket::{http::Status, response::status::Custom, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::{Client, NoTls};
use rocket::form::Form;





#[derive(Serialize, Deserialize, Clone, Validate)]
struct NewUserData {
    id: Option<i32>,
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn add_user(
    new_user: Json<NewUser>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {
    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;
    let secret = state.secret.clone();
    db.run(move |conn| {
        database::users::create(conn, &username, &email, &password)
            .map(|user| json!({ "user": user.to_user_auth(&secret) }))
            .map_err(|error| {
                let field = match error {
                    UserCreationError::DuplicatedEmail => "email",
                    UserCreationError::DuplicatedUsername => "username",
                };
                Errors::new(&[(field, "has already been taken")])
            })
    })
    .await
}

#[get("/get_users?<username>&<user_id>&<email>")]
pub async fn get_users(db: Db, user_id: i32, username: String, email: String) -> Result<Value, Errors> {
    // let filters = filters.map(|f| f.into_inner());
    db.run(move |conn| {
        database::users::get_users(
            conn,

            Some(UserFiltering {
                id: user_id,
                username,
                email,
            })

            // filters.map(|f| User {
            //     username: f.username,
            //     email: f.email,
            //     id: f.id,
            //     image: f.image,
            //     hash: f.hash,
            // }),
        )
        .map(|users| json!({ "users": users }))
        .map_err(|_| Errors::new(&[("database", "failed to fetch users")]))
    })
    .await

    // Ok(format!("Users"))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user: database::users::UpdateUserData,
}

// #[put("/user", format = "json", data = "<user>")]
// pub async fn put_user(
//     user: Json<UpdateUser>,
//     user_id: i32,
//     db: Db,
//     state: &State<AppState>,
// ) -> Option<User> {
//     let secret = state.secret.clone();
//     db.run(move |conn| database::users::update(conn, user_id, &user.user))
//         .await
//         .map(|user| json!({ "user": user.to_user_auth(&secret) }))
// }

// #[put("/api/users/<id>", data = "<user>")]
// pub async fn update_user(
//     conn: &State<Client>,
//     id: i32,
//     user: Json<User>
// ) -> Result<Json<Vec<User>>, Custom<String>> {
//     execute_query(
//         conn,
//         "UPDATE users SET name = $1, email = $2 WHERE id = $3",
//         &[&user.name, &user.email, &id]
//     ).await?;
//     get_users(conn).await
// }

// #[delete("/api/users/<id>")]
// async fn delete_user(conn: &State<Client>, id: i32) -> Result<Status, Custom<String>> {
//     execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await?;
//     Ok(Status::NoContent)
// }

// async fn execute_query(
//     client: &Client,
//     query: &str,
//     params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
// ) -> Result<u64, Custom<String>> {
//     client
//         .execute(query, params).await
//         .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
// }
