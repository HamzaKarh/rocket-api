#![feature(proc_macro_hygiene, decl_macro)]
mod schema; // im guessing this is a schema containing every entity model
            // mod paste_id;

#[macro_use]
extern crate diesel;
mod db_connect;
#[path = "entities/files/file.rs"]
mod files;

use db_connect::*;
use rocket::{self, get, routes};

// use crate::db_connect::establish_connection;
use files::{static_rocket_route_info_for_create_file, static_rocket_route_info_for_get_all_files};

// routes is imported from rocket crate
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/files", routes![create_file, get_all_files])
        .launch();
}
