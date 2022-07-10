#![feature(proc_macro_hygiene, decl_macro)]
mod schema; // im guessing this is a schema containing every entity model
            // mod paste_id;

#[macro_use]
extern crate diesel;
mod db_connect;
#[path = "entities/files/file.rs"]
mod files;

use db_connect::*;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{self, get, post, routes};
use std::env;

// use crate::db_connect::establish_connection;
use crate::files::{File, NewFile};
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::json::Json;
use std::fmt::format;

// routes is imported from rocket crate
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    // let connexion = DbConn(establish_connection());
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .launch();
}
