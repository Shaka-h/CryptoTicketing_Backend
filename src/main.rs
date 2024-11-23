// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]


use rocket;
use BackEnd;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = BackEnd::rocket().launch().await?;

    Ok(())
}