use bytes::{Bytes, Buf, BufMut};
use std::io::Cursor;
use ring::{digest, signature};
use untrusted::Input;

use crate::util::*;
use crate::u2ferror::U2fError;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, U2fError>;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    pub counter: u32,
    pub user_presence: bool,
}

pub fn parse_sign_response(app_id: String, client_data: Vec<u8>, public_key: Vec<u8>, sign_data: Vec<u8>) -> Result<Authorization> { 
    if get_user_presence(&sign_data[..]) != 1 {
        return Err(U2fError::InvalidUserPresenceByte);
    }    
    
    //Start parsing ... 
    let mut mem = Bytes::from(sign_data);
    let user_presence_flag = mem.split_to(1);
    let counter = mem.split_to(4);
    
    let sig_len = asn_length(mem.clone()).unwrap();
    let signature = mem.split_to(sig_len);

    // Let's build the msg to verify the signature
    let app_id_hash = digest::digest(&digest::SHA256, &app_id.into_bytes());
    let client_data_hash = digest::digest(&digest::SHA256, &client_data[..]);

    let mut msg = vec![];
    msg.put(app_id_hash.as_ref());
    msg.put(user_presence_flag.clone()); 
    msg.put(counter.clone());  
    msg.put(client_data_hash.as_ref());

    let input_sig = Input::from(&signature[..]);
    let input_msg = Input::from(msg.as_ref());
    let input_public_key = Input::from(&public_key[..]);

    // The signature is to be verified by the relying party using the public key obtained during registration.
    let _ = signature::verify(&signature::ECDSA_P256_SHA256_ASN1, input_public_key, input_msg, input_sig);

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
    buf.get_u32_be()
}