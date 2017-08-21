#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate u2f;
extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

use std::io;

use rocket_contrib::{Json, Value};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/register_request", format = "application/json")]
fn register_request() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, register_request])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}