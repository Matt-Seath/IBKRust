#[macro_use]
extern crate rocket;

/// Handles GET requests to the root path ("/").
///
/// Returns a static string with the message "Hello, World!".
#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<name>")]
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Launches the Rocket web server with the provided routes.
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, greet])
        .launch()
        .await
        .map_err(|err| println!("{:?}", err))
        .ok();
}
