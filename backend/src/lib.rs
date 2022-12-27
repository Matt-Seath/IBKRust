#[macro_use]
extern crate rocket;

use crate::routes::home;
use rocket::{Build, Rocket};

pub mod routes;

pub async fn setup_rocket() -> Rocket<Build> {
    let my_rocket = rocket::build().mount("/", routes![home]);
    my_rocket
}
