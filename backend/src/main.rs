#[macro_use]
extern crate rocket;

/// Handles GET requests to the root path ("/").
///
/// Returns a static string with the message "Hello, World!".
#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

/// Launches the Rocket web server with the provided routes.
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
