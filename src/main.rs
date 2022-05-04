#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "ok!"
}

#[get("/obj/<filename>")]
fn get_file(filename: &str) -> String {
    format!("filename: {}", filename)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_file])
}
