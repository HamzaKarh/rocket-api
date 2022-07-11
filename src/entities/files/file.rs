// use crate::db_connect::establish_connection;
#[path = "../../schema.rs"]
mod schema;
use crate::db_connect;
use crate::schema::files;
use db_connect::DbConn;
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::{self, delete, get, post};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct File {
    pub id: i32,
    pub title: String,
    pub dir: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub title: String,
    pub dir: String,
}

#[post("/create", data = "<new_file>")]
pub fn create_file(conn: DbConn, new_file: Json<NewFile>) -> Json<File> {
    let result = diesel::insert_into(files::table)
        .values(&new_file.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[get("/")]
pub fn get_all_files(conn: DbConn) -> Json<Vec<File>> {
    let result = files::table
        .order(files::columns::id::desc(Default::default()))
        .load::<File>(&*conn)
        .unwrap();

    Json(result)
}
