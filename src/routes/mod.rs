use std::env;
use std::io;
use std::path::Path;

use rocket::fs::NamedFile;

#[get("/")]
pub async fn home() -> io::Result<NamedFile> {
    let page_directory_path = get_directory_path();
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await
}

fn get_directory_path() -> String {
    format!("{}/frontend/templates/", env!("CARGO_MANIFEST_DIR"))
}
