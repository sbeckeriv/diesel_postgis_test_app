#![feature(proc_macro)]
#[macro_use]
extern crate diesel;
extern crate diesel_postgis;
extern crate dotenv;
use diesel_postgis::Geometry;

#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_postgis::Point;
use dotenv::dotenv;
use std::env;
fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    println!("Hello, world!");
}
