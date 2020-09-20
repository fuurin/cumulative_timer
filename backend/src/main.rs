#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod cors;
pub mod db;
pub mod users_controller;

use cors::CorsFairing;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, users_controller::index, users_controller::show])
        .attach(CorsFairing)
        .launch();
}
