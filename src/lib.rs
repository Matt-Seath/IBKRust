#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::routes::{files, home};
use mysql::prelude::*;
use mysql::*;
use rocket::{Build, Rocket};
use std::env;

pub mod routes;

pub async fn setup_rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    let mysql_url: String = env::var("MYSQL_URL")
        .expect("mysql_url variable should be set in an '.env' file in project root directory");
        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        
    
    for (k, v) in env::vars() {
        eprintln!("{}={}", k, v);
    }
    let my_rocket = rocket::build().mount("/", routes![home, files]);
    my_rocket
}
