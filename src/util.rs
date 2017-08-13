use base64::{encode};
use ring::rand::{SecureRandom, SystemRandom};

pub const U2F_V2: &'static str = "U2F_V2";

// Generates a challenge from a secure, random source.
pub fn generate_challenge(size: usize) -> Result<Vec<u8>, String> {
    let mut bytes: Vec<u8> = vec![0; size];

    let rng = SystemRandom::new();
    rng.fill(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

