// use crate::db_connect::establish_connection;
#[path = "../../schema.rs"]
mod schema;
use crate::db_connect;
use crate::schema::files;
use db_connect::DbConn;
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::{self, delete, get, post};
use rocket::http::hyper::header::ContentType;
use rocket::http::uri::Path;
use rocket::response::content::Html;
use rocket_contrib::json::Json;
use rocket_upload::MultipartDatas;
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
pub fn create_file(conn: DbConn, new_file: MultipartDatas) -> Json<File> {
    let file_fields = new_file.texts;
    let result = diesel::insert_into(files::table)
        .values(&file_fields)
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

#[post("/upload", data = "<data>")]
fn upload(content_type: &ContentType, data: MultipartDatas) -> Html<String> {
    // to get a param from client
    // let mut result=format!("UserID:{}<br>",userid);
    // content_type not used here, just for more informations
    let mut result = format!("{:?}<br>",content_type);
    // aquire all Form field data
    for t in data.texts {
    result = format!("{}FieldName:{} --- FieldValue:{}<br>",result,t.key,t.value);
    }
    // aquire all files upload
    for f in data.files {
    result = format!("{}FieldName:{} --- FileName:{} --- StoragePath:{}<br>",
      result,f.name,f.filename,f.path);
    f.persist(Path::new("upload"));
    }
    Html(format!("<html><head></head><body>upload coming...<br>{}</body></html>",result))
}
