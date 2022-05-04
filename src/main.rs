#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "ok!"
}

#[get("/obj/<filename>")]
async fn get_file(filename: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("/tmp/sky/").join(filename);
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_file])
}
