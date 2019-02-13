use crate::util::*;
use crate::messages::*;
use crate::register::*;
use crate::authorization::*;

use base64::{encode, decode_config, Config, CharacterSet};
use chrono::prelude::*;
use time::Duration;
use crate::u2ferror::U2fError;

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
        if expiration(challenge.timestamp) > Duration::seconds(300) {
            return Err(U2fError::ChallengeExpired);
        }
    
        let config = Config::new(CharacterSet::UrlSafe, true);

        let registration_data: Vec<u8> = decode_config(&response.registration_data[..], config).unwrap();
        let client_data: Vec<u8> = decode_config(&response.client_data[..], config).unwrap();

        parse_registration(challenge.app_id, client_data, registration_data)
    }

    fn registered_keys(&self, registrations: Vec<Registration>) -> Vec<RegisteredKey> {
        let mut keys: Vec<RegisteredKey> = vec![];

        for registration in registrations {
            keys.push(get_registered_key(self.app_id.clone(), registration.key_handle));
        }

        keys
    }
    
    pub fn sign_request(&self, challenge: Challenge, registrations: Vec<Registration>) -> U2fSignRequest {
        let mut keys: Vec<RegisteredKey> = vec![];

        for registration in registrations {
            keys.push(get_registered_key(self.app_id.clone(), registration.key_handle));
        }

        let signed_request = U2fSignRequest {
            app_id : self.app_id.clone(),
            challenge: encode(challenge.challenge.as_bytes()),
            registered_keys: keys
        };

        signed_request
    }  

    pub fn sign_response(&self, challenge: Challenge, reg: Registration, sign_resp: SignResponse, counter: u32) -> Result<u32> {
        if expiration(challenge.timestamp) > Duration::seconds(300) {
            return Err(U2fError::ChallengeExpired);
        }

        if sign_resp.key_handle != get_encoded(&reg.key_handle[..]) {            
            return Err(U2fError::WrongKeyHandler);
        }

        let config = Config::new(CharacterSet::UrlSafe, true);
        let client_data: Vec<u8> = decode_config(&sign_resp.client_data[..], config).map_err(|_e| U2fError::InvalidClientData)?;
        let sign_data: Vec<u8> = decode_config(&sign_resp.signature_data[..], config).map_err(|_e| U2fError::InvalidSignatureData)?;

        let public_key = reg.pub_key;

        let auth = parse_sign_response(self.app_id.clone(), client_data.clone(), public_key, sign_data.clone());

        match auth {
            Ok(ref res) => {
                // CounterTooLow is raised when the counter value received from the device is
                // lower than last stored counter value.
                if res.counter < counter {
                    return Err(U2fError::CounterTooLow);
                }
                else {
                    return Ok(res.counter);
                }
            },
                Err(e) => return Err(e),
            }
        }       
}