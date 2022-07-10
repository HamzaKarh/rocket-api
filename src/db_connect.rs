extern crate dotenv;

use dotenv::dotenv;
use rocket_contrib::databases::{database, diesel::Connection, diesel::PgConnection};
use std::env;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }
