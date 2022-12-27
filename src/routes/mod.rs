use std::env;
use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

#[get("/")]
pub async fn home() -> io::Result<NamedFile> {
    let page_directory_path = get_directory_path();
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = get_directory_path();
    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}

fn get_directory_path() -> String {
    format!("{}/frontend/build", env!("CARGO_MANIFEST_DIR"))
}
