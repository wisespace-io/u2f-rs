use util::*;

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

    pub fn request(&self) -> Result<Challenge, String> {

        let challenge = Challenge {
            challenge : generate_challenge(32)?,
            timestamp : Utc::now(),
            app_id : self.app_id.clone()
        };

        Ok(challenge)
    }
}