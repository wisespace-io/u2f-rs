#![feature(proc_macro_hygiene, decl_macro)]

extern crate u2f;
extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate lazy_static;
extern crate rocket_contrib;

use std::io;

use u2f::protocol::*;
use u2f::messages::*;
use u2f::register::*;

use rocket::{State, catch, catchers, get, post, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};

use std::error::Error;
use std::sync::Mutex;

static APP_ID : &'static str = "https://localhost:30443";

lazy_static! {
    // In a real application this could be a database lookup.
    static ref REGISTRATIONS: Mutex<Vec<Registration>> = {
        let registrations: Mutex<Vec<Registration>> = Mutex::new(vec![]);
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
    let u2f_request = state.u2f.request(challenge.clone(), REGISTRATIONS.lock().unwrap().clone());

    Json(u2f_request.unwrap())
}

#[post("/api/register_response", format = "application/json", data = "<response>")]
fn register_response(mut cookies: Cookies, response: Json<RegisterResponse>, state: State<U2fClient>) -> Result<JsonValue, NotFound<String>> {

    let cookie = cookies.get_private("challenge");

    if let Some(ref cookie) = cookie {
        let challenge: Challenge = serde_json::from_str(cookie.value()).unwrap();
        let registration = state.u2f.register_response(challenge, response.into_inner());
        match registration {
            Ok(reg) =>  {
                REGISTRATIONS.lock().unwrap().push(reg);
                cookies.remove_private(Cookie::named("challenge"));
                return Ok(json!({"status": "success"}));
            },
            Err(e) => {
                return Err(NotFound(format!("{:?}", e.description())));
            }
        }
    } else {
        return Err(NotFound(format!("Not able to recover challenge")));
    }
}

#[get("/api/sign_request", format = "application/json")]
fn sign_request(mut cookies: Cookies, state: State<U2fClient>) -> Json<U2fSignRequest> {
    let challenge = state.u2f.generate_challenge().unwrap();
    let challenge_str = serde_json::to_string(&challenge);

    // Only for this demo we will keep the challenge in a private (encrypted) cookie
    cookies.add_private(Cookie::new("challenge", challenge_str.unwrap()));

    let signed_request = state.u2f.sign_request(challenge, REGISTRATIONS.lock().unwrap().clone());

    return Json(signed_request);
}

#[post("/api/sign_response", format = "application/json", data = "<response>")]
fn sign_response(mut cookies: Cookies, response: Json<SignResponse>, state: State<U2fClient>) -> Result<JsonValue, NotFound<String>> {
    let cookie = cookies.get_private("challenge");
    if let Some(ref cookie) = cookie {
        let challenge: Challenge = serde_json::from_str(cookie.value()).unwrap();

        let registrations = REGISTRATIONS.lock().unwrap().clone();
        let sign_resp = response.into_inner();

        let mut _counter: u32 = 0;
        for registration in registrations {
            let response = state.u2f.sign_response(challenge.clone(), registration, sign_resp.clone(), _counter);
            match response {
                Ok(new_counter) =>  {
                    _counter = new_counter;
                    return Ok(json!({"status": "success"}));
                },
                Err(_e) => {
                    break;
                }
            }
        }
        return Err(NotFound(format!("error verifying response")));
    } else {
        return Err(NotFound(format!("Not able to recover challenge")));
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    let u2f_client = U2fClient {
        u2f: U2f::new(APP_ID.into())
    };

    rocket::ignite()
        .mount("/", routes![index, register_request, register_response, sign_request, sign_response])
        .register(catchers![not_found])
        .manage(u2f_client)
}

fn main() {
    rocket().launch();
}