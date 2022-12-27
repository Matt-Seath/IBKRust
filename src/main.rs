#[macro_use]
extern crate rocket;
extern crate dotenv;

use ibkrust::setup_rocket;
use rocket::{Build, Rocket};
use dotenv::dotenv;
use std::env;



/// Launches the Rocket web server.
#[launch]
async fn rocket() -> Rocket<Build> {
    setup_rocket().await
}
