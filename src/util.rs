use chrono::prelude::*;
use time::Duration;
use ring::rand::{SecureRandom, SystemRandom};
use bytes::{Bytes, Buf};
use std::io::Cursor;

use u2ferror::U2fError;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, U2fError>;

pub const U2F_V2: &'static str = "U2F_V2";

// Generates a challenge from a secure, random source.
pub fn generate_challenge(size: usize) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = vec![0; size];

    let rng = SystemRandom::new();
    rng.fill(&mut bytes).map_err(|_e| U2fError::RandomSecureBytesError)?;
    Ok(bytes)
}

pub fn expiration(timestamp: String) -> Duration {
    let now: DateTime<Utc> = Utc::now();

    let ts = timestamp.parse::<DateTime<Utc>>();

    now.signed_duration_since(ts.unwrap())
}

// Decode initial bytes of buffer as ASN and return the length of the encoded structure.
// http://en.wikipedia.org/wiki/X.690
pub fn asn_length(mem: Bytes) -> Result<usize> {
    let mut buf = Cursor::new(mem);

    let _ = buf.get_u8(); // skip verification byte

    let b = buf.get_u8();
    if b & 0x80 == 0 {
        return Ok((b & 0x7f) as usize);
    }

    let numbem_of_bytes = b & 0x7f;
    if numbem_of_bytes == 0 {
        return Err(U2fError::Asm1DecoderError);
    }

    let mut length = 0;
    for _ in 0..numbem_of_bytes {
        let b = buf.get_u8();
        if length > (usize::max_value() >> 8) {
            return Err(U2fError::Asm1DecoderError);
        }
        length <<= 8;
        length |= b as usize;
        if length == 0 {
            return Err(U2fError::Asm1DecoderError);
        }
    }

    if length < 128 {
        return Err(U2fError::Asm1DecoderError);
    }

    Ok(length)
}