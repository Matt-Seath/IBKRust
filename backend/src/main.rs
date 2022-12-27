#[macro_use]
extern crate rocket;

use ibkrust::setup_rocket;
use rocket::{Build, Rocket};

/// Launches the Rocket web server.
#[launch]
async fn rocket() -> Rocket<Build> {
    setup_rocket().await
}
