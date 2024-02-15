mod models;
mod db;


use std::fs;


use crate::models::{DBState, Epic, Status, Story};


fn main() {
    let file = fs::read_to_string("./db/db.json").unwrap();
    let parsed: DBState = serde_json::from_str(&file).unwrap();
    println!("{parsed:?}")
}
