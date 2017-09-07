use util::*;
use messages::*;
use register::*;

use base64::{encode, decode, decode_config, Config, CharacterSet, LineWrap};
use chrono::prelude::*;
use time::Duration;
use u2ferror::U2fError;

type Result<T> = ::std::result::Result<T, U2fError>;

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

    pub fn generate_challenge(&self) -> Result<Challenge> {
        let utc: DateTime<Utc> = Utc::now();

        let challenge_bytes = generate_challenge(32)?; 
        let challenge = Challenge {
            challenge : encode(&challenge_bytes),
            timestamp : format!("{:?}", utc),
            app_id : self.app_id.clone()
        };
        
        Ok(challenge.clone())
    }

    pub fn request(&self, challenge: Challenge, registrations: Vec<Registration>) -> Result<U2fRegisterRequest> {
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

    pub fn register_response(&self, challenge: Challenge, response: RegisterResponse) -> Result<Registration> {
        let now: DateTime<Utc> = Utc::now();

        let timestamp = challenge.timestamp.parse::<DateTime<Utc>>();

	    let expiration = now.signed_duration_since(timestamp.unwrap());

        if expiration > Duration::seconds(300) {
            return Err(U2fError::ChallengeExpired);
        }
    
        let config = Config::new(CharacterSet::UrlSafe, false, false, LineWrap::NoWrap);

        let registration_data: Vec<u8> = decode_config(&response.registration_data[..], config).unwrap();
        let client_data: Vec<u8> = decode_config(&response.client_data[..], config).unwrap();

        parse_registration(challenge.app_id, client_data, registration_data)
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