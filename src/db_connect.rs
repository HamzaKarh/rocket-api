extern crate dotenv;

use rocket_contrib::databases::database;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);
