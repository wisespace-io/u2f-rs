use util::*;
use messages::*;
use register::*;

use base64::{encode, decode};
use chrono::prelude::*;

#[derive(Clone)]
pub struct U2f {
    app_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub app_id: String,
    pub challenge: String,
    pub timestamp: String,
}

impl Challenge {
    pub fn new() -> Self {
        Challenge {
            app_id: String::new(),
            challenge: String::new(),
            timestamp: String::new()
        }
    }
}

impl U2f {
    // The app ID is a string used to uniquely identify an U2F app
    pub fn new(app_id: String) -> Self {
        U2f {
            app_id: app_id,
        }
    }

    pub fn generate_challenge(&self) -> Result<Challenge, String> {
        
        let challenge_bytes = generate_challenge(32)?; 
        let challenge = Challenge {
            challenge : encode(&challenge_bytes),
            timestamp : Utc::now().to_string(),
            app_id : self.app_id.clone()
        };
        
        Ok(challenge.clone())
    }

    pub fn request(&self, challenge: Challenge, registrations: Vec<Registration>) -> Result<U2fRegisterRequest, String> {
        let u2f_request = U2fRegisterRequest {
            app_id : self.app_id.clone(),
            register_requests: self.register_request(challenge),
            registered_keys: self.registered_keys(registrations)
        };

        Ok(u2f_request)
    }

    fn register_request(&self, challenge: Challenge) -> Vec<RegisterRequest> {
        let mut requests: Vec<RegisterRequest> = vec![];

        let request = RegisterRequest {
            version : U2F_V2.into(),
            challenge: challenge.challenge
        };
        requests.push(request);

        requests
    }

    pub fn register_response(&self, challenge: Challenge, response: RegisterResponse) -> Registration {
        //let now = Utc::now();
        //let dt2 = DateTime::parse_from_str(challenge.timestamp.as_str(), "%Y-%m-%d %H:%M:%S.%f UTC");
	    //let passed = now.signed_duration_since(dt2.unwrap());

        let registration_data: Vec<u8> = decode(&response.registration_data[..]).unwrap();
        let client_data: Vec<u8> = decode(&response.client_data[..]).unwrap();
        parse_registration(registration_data)
    }

    fn registered_keys(&self, registrations: Vec<Registration>) -> Vec<RegisteredKey> {
        let mut keys: Vec<RegisteredKey> = vec![];

        for registration in registrations {
            let key = RegisteredKey {
                version : U2F_V2.into(),
                key_handle: Some(encode(&registration.key_handle)),
                app_id: self.app_id.clone(),
            };
            keys.push(key);
        }

        keys
    }
}