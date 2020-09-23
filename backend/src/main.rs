#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod cors;
pub mod db;
pub mod controllers;

use cors::CorsFairing;
use controllers::users;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, users::index, users::show])
        .attach(CorsFairing)
        .launch();
}
