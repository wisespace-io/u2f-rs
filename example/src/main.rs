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
use u2f::register::*;

use rocket::State;
use rocket_contrib::{Json, Value};
use rocket::response::NamedFile;

static APP_ID : &'static str = "https://localhost:30443";

struct StateVariables {
    pub u2f: U2f,
    pub challenge: Challenge,
    pub registrations: Vec<Registration>,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/api/register_request", format = "application/json")]
fn register_request(state: State<StateVariables>) -> Json<U2fRegisterRequest> {
    // Only for this demo we will keep the challenge in memory
    let challenge = state.u2f.new_challenge().unwrap();
    // Send registration request to the browser.
    let u2f_request = state.u2f.request(challenge.clone(), state.registrations.clone());

    Json(u2f_request.unwrap())
}

#[post("/api/register_response", format = "application/json", data = "<response>")]
fn register_response(response: Json<RegisterResponse>, state: State<StateVariables>) -> Json<Value> {
    if response.challenge.is_empty() {
        return Json(json!({"status": "error", "reason": "Challenge is missing"}));
    }

    //let test = state.u2f.register_response(state.challenge.clone(), response.into_inner());

    Json(json!({"status": "ok"}))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    // Normally registrations variable should be stored in a database, keeping it in memory for demo purpose.
    let registrations: Vec<Registration> = vec![];
    let challenge = Challenge {app_id: String::new(), timestamp: String::new(), challenge: vec![]};

    let state_variables = StateVariables {
        registrations: registrations,
        u2f: U2f::new(APP_ID.into()),
        challenge: challenge,
    };

    rocket::ignite()
        .mount("/", routes![index, register_request, register_response])
        .catch(errors![not_found])
        .manage(state_variables)
}

fn main() {
    rocket().launch();
}