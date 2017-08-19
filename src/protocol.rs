use util::*;
use messages::*;

use base64::{encode};
use chrono::prelude::*;

#[derive(Clone)]
pub struct U2f {
    app_id: String,
}

#[derive(Debug, Clone)]
pub struct Challenge {
    app_id: String,
    challenge: Vec<u8>,
    timestamp: DateTime<Utc>,
}

impl U2f {
    // The app ID is a string used to uniquely identify an U2F app
    pub fn new(app_id: String) -> Self {
        U2f {
            app_id: app_id,
        }
    }

    pub fn request(&self) -> Result<U2fRegisterRequest, String> {

        let challenge = Challenge {
            challenge : generate_challenge(32)?,
            timestamp : Utc::now(),
            app_id : self.app_id.clone()
        };

        let u2f_request = U2fRegisterRequest {
            app_id : self.app_id.clone(),
            register_requests: self.register_request(challenge),
            registered_keys: self.registered_keys() 
        };

        Ok(u2f_request)
    }

    fn register_request(&self, challenge: Challenge) -> Vec<RegisterRequest> {
        let mut requests: Vec<RegisterRequest> = vec![];

        let request = RegisterRequest {
            version : U2F_V2.into(),
            challenge: encode(&challenge.challenge)
        };
        requests.push(request);

        requests
    }

    fn registered_keys(&self) -> Vec<RegisteredKey> {
        let mut keys: Vec<RegisteredKey> = vec![];

        let key = RegisteredKey {
            version : U2F_V2.into(),
            key_handle: None,
            app_id: self.app_id.clone(),
        };
        keys.push(key);

        keys
    }
}