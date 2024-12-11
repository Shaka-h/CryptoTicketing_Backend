use crate::models::user::{User, UserFiltering};
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &mut PgConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .expect("hash error")
        .to_string()
        .to_owned();

    let new_user = &NewUser {
        username,
        email,
        hash: &hash[..],
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn login(conn: &mut PgConnection, email: &str, password: &str) -> Option<User> {
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let parsed_hash = PasswordHash::new(&user.hash).unwrap();
    let password_matches = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .is_ok();

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub fn get_users(
    conn: &mut PgConnection,
    filters: Option<UserFiltering>,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    // let mut query = users.into_boxed();
    println!("filtlers {:?}", filters);

    if let Some(f) = filters {
        let mut query = users.into_boxed();
        if let Some(ref username_filter) = f.username  {
            query = query.filter(username.eq(username_filter));
        }
        if let Some(ref email_filter) = f.email {
            query = query.filter(email.eq(email_filter));
        }
        if let Some(id_filter) = f.id {
            query = query.filter(id.eq(id_filter));
        }
        if let Some(limit_filter) = f.limit {
            query = query.limit(limit_filter);
        }
        //In Rust, integers (like i32, u32, etc.) cannot be null because Rust does not allow null values in its type system.
        //Instead, Rust uses the Option type to represent values that may or may not be present.
        //For example, an integer that might be "null" would be represented as Option<i32> or Option<u32>.
        
        query
            // .limit(5) // Limit to 5 results
            .load::<User>(conn) // Load results into Vec<User>
    } else {
        let results = users
            // .limit(5)
            // .select(User::as_select())
            .load::<User>(conn) // Load results into Vec<User>
            .expect("Error loading users");

        println!("Displaying {} users", results.len());
        for user in &results {
            println!("{}", user.username);
            println!("-----------\n");
            println!("{}", user.email);
        }
        Ok(results)
    }
}

pub fn find(conn: &mut PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone, Validate)]
#[table_name = "users"]
pub struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,
    image: Option<String>,

    // hack to skip the field
    #[column_name = "hash"]
    password: Option<String>,
}

pub fn update(conn: &mut PgConnection, id: i32, data: &UpdateUserData) -> Option<User> {
    let data = &UpdateUserData {
        password: None,
        ..data.clone()
    };
    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}

pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, Error>{
    let user_deleted = diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(conn)
        .expect("Error deleting user");

    println!("Deleted {} user", user_deleted);

    Ok(user_deleted)
}
