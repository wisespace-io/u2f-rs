use bytes::{Bytes, Buf, BufMut, BigEndian};
use std::io::Cursor;
use ring::{error, signature};
use untrusted::Input;
use crypto_hash::{Algorithm, hex_digest};

use util::*;
use u2ferror::U2fError;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, U2fError>;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    pub counter: u32,
    pub user_presence: bool,
}

pub fn parse_sign_response(app_id: String, client_data: Vec<u8>, pub_key: Vec<u8>, sign_data: Vec<u8>) -> Result<Authorization> {    
    
    if get_user_presence(&sign_data[..]) != 1 {
        return Err(U2fError::InvalidUserPresenceByte);
    }    
    
    //Start parsing ... 
    let mut mem = Bytes::from(sign_data);
    let _ = mem.split_to(1); // advance the user presence byte.
    let counter = mem.split_to(4);
    let signature_len = asn_length(mem.clone()).unwrap();
    let signature = mem.split_to(signature_len as usize);

    let app_id_hash = hex_digest(Algorithm::SHA256, &app_id.into_bytes());
    let client_data_hash = hex_digest(Algorithm::SHA256, &client_data);

    let mut sign_base = vec![];
    sign_base.put(app_id_hash);
    sign_base.put(client_data_hash);
    sign_base.put(counter.clone());  

    let input_pub_key = Input::from(&pub_key[..]);
    let input_sign = Input::from(&signature[..]);
    let input_sign_base = Input::from(&sign_base[..]);
    signature::verify(&signature::ECDSA_P256_SHA256_FIXED, input_pub_key, input_sign_base, input_sign)
        .map_err(|error::Unspecified| U2fError::BadSignature)?;

    let authorization = Authorization {
        counter: get_counter(counter),
        user_presence: true
    };

    Ok(authorization)
}

fn get_user_presence(user_presence: &[u8]) -> u8 {
    let mut buf = Cursor::new(user_presence);
    buf.get_u8()
}

fn get_counter(counter: Bytes) -> u32 {
    let mut buf = Cursor::new(&counter[..]);
    buf.get_u32::<BigEndian>()
}