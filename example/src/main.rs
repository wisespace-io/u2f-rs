#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate u2f;
extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

use std::io;

use u2f::protocol::*;
use u2f::messages::*;

use rocket::State;
use rocket_contrib::{Json, Value};
use rocket::response::NamedFile;

static APP_ID : &'static str = "https://localhost:30443";

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/register_request", format = "application/json")]
fn register_request(u2f: State<U2f>) -> Json<U2fRegisterRequest> {
    
    // Send registration request to the browser.
    let u2f_request = u2f.request();
    Json(u2f_request.unwrap())
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    let u2f = U2f::new(APP_ID.into());

    rocket::ignite()
        .mount("/", routes![index, register_request])
        .catch(errors![not_found])
        .manage(u2f)
}

fn main() {
    rocket().launch();
}