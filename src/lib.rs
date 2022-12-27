#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::routes::home;
use rocket::{Build, Rocket};

pub mod routes;

pub async fn setup_rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    for (k, v) in std::env::vars() {
        eprintln!("{}={}", k, v);
    }
    let my_rocket = rocket::build().mount("/", routes![home]);
    my_rocket
}
