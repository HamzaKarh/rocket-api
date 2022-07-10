use std::fmt::format;

use crate::db_connect::establish_connection;
use crate::schema::files::dsl::*;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use rocket_upload::MultipartDatas;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct File {
    pub id: i32,
    pub title: String,
    pub dir: String,
}

pub struct NewFile {
    pub title: String,
    pub dir: String,
}

impl File {
    pub fn get_by_id(index: i32) -> File {
        let connection = establish_connection();
        let result = files
            .filter(id.eq(index))
            // .limit(5)
            .load::<File>(&connection)
            .expect("Error loading files");
        result[0][0]
        // println!("Displaying {} posts", results.len());
    }

    pub fn get_by_dir(full_path: String) -> File {
        let connection = establish_connection();
        let split_path: Vec<&str> = full_path.split("/").collect();
        let file_name = String::from(split_path[split_path.len() - 1]);
        let file_dir = full_path.replace(&file_name, "");
        let result = files
            .filter(dir.eq(file_dir))
            // .limit(5)
            .load::<File>(&connection)
            .expect("Error loading files");
        result[0][0]
    }

    // pub fn new(){
    //
    // }

    // pub fn upload(&self){
    //
    // }

    // pub fn delete(&self){
    //
    // }

    // pub fn rename(&self){
    //
    // }
}
