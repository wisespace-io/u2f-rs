#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate u2f;
extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket_contrib;

use std::io;

use u2f::protocol::*;
use u2f::messages::*;
use u2f::register::*;

use rocket::State;
use rocket_contrib::{Json, Value};
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};

static APP_ID : &'static str = "https://localhost:30443";

lazy_static! {
    // In a real application this could be a database lookup.
    static ref REGISTRATIONS: Vec<Registration> = {
        let registrations: Vec<Registration> = vec![];
        registrations
    };
}

struct U2fClient {
    pub u2f: U2f
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/register_request", format = "application/json")]
fn register_request(mut cookies: Cookies, state: State<U2fClient>) -> Json<U2fRegisterRequest> {
    let challenge = state.u2f.generate_challenge().unwrap();
    let challenge_str = serde_json::to_string(&challenge);

    // Only for this demo we will keep the challenge in a private (encrypted) cookie
    cookies.add_private(Cookie::new("challenge", challenge_str.unwrap()));

    // Send registration request to the browser.
    let u2f_request = state.u2f.request(challenge.clone(), REGISTRATIONS.clone());

    Json(u2f_request.unwrap())
}

#[post("/api/register_response", format = "application/json", data = "<response>")]
fn register_response(mut cookies: Cookies, response: Json<RegisterResponse>, state: State<U2fClient>) -> Json<Value> {
    if response.challenge.is_empty() {
        return Json(json!({"status": "error", "reason": "Challenge is missing"}));
    }

    let cookie = cookies.get_private("challenge");

    if let Some(ref cookie) = cookie {
        let challenge: Challenge = serde_json::from_str(cookie.value()).unwrap();
        let registration = state.u2f.register_response(challenge, response.into_inner());
    }

    Json(json!({"status": "success"}))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    let u2f_client = U2fClient {
        u2f: U2f::new(APP_ID.into())
    };

    rocket::ignite()
        .mount("/", routes![index, register_request, register_response])
        .catch(errors![not_found])
        .manage(u2f_client)
}

fn main() {
    rocket().launch();
}