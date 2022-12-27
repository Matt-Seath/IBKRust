#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    email: String,
    active: bool,
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e3dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                uuid: String::from("3e3dd4ae-3c37-40c6-aa64-7061f284ce28"),
                name: String::from("John Doe"),
                email: String::from("johndoe@gmail.com"),
                active: true,
            },
        );
        map
    };
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<name>")]
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/user/<uuid>", rank = 1, format = "text/html")]
async fn user(uuid: &str) -> String {
    let user = USERS.get(uuid);
    match user {
        Some(u) => format!("Found user: {:?}", u),
        None => String::from("User not found"),
    }
}

/// Launches the Rocket web server with the provided routes.
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, greet, user])
        .launch()
        .await
        .map_err(|err| println!("{:?}", err))
        .ok();
}
