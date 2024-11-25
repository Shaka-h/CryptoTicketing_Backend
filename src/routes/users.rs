// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rocket::serde::{
    Deserialize, Serialize,
    json::{json, Json, Value},
};
use crate::database::{self, users::UserCreationError, Db};
use crate::errors::{Errors, FieldValidator};
use crate::{
    config::AppState,
    models::user::UserFiltering,
};
use rocket::State;

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
pub async fn get_users(
    db: Db,
    user_id: i32,
    username: String,
    email: String,
) -> Result<Value, Errors> {
    // let filters = filters.map(|f| f.into_inner());
    db.run(move |conn| {
        database::users::get_users(
            conn,
            Some(UserFiltering {
                id: user_id,
                username,
                email,
                limit: 100,
            }),
        )
        .map(|users| json!({ "users": users }))
        .map_err(|_| Errors::new(&[("database", "failed to fetch users")]))
    })
    .await

    // Ok(format!("Users"))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    id: i32,
    user: database::users::UpdateUserData,
}

#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(user: Json<UpdateUser>, db: Db) -> Option<Value> {
    db.run(move |conn| database::users::update(conn, user.id, &user.user))
        .await
        .map(|user| json!({ "user": user }))
}

#[derive(Deserialize)]
pub struct DeleteUser {
    id: i32,
}

#[delete("/user", format = "json", data = "<user>")]
pub async fn delete_user(user: Json<DeleteUser>, db: Db) -> Result<Value, Errors> {
    db.run(move |conn| database::users::delete(conn, user.id))
        .await
        .map(|_| json!({ "message": "User deleted successfully" }))
        .map_err(|_| Errors::new(&[("database", "failed to delete user")]))
}
