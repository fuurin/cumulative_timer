#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod cors;

use models::User;
use schema::users::dsl::*;
use cors::CorsFairing;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use rocket_contrib::json::Json;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn users_index() -> Json<Vec<User>> {
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
    Json(results)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, users_index])
        .attach(CorsFairing)
        .launch();
}
