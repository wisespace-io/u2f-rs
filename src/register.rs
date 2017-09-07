use bytes::{Bytes, Buf, BufMut};
use std::io::Cursor;
use std::str;
use ring::{error, signature};
use untrusted::Input;
use byteorder::{ByteOrder, BigEndian};
use boringauth::oath::{TOTPBuilder, HashFunction};

use u2ferror::U2fError;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, U2fError>;

// Single enrolment or pairing between an application and a token.
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub key_handle: Vec<u8>,
	pub pub_key: Vec<u8>,

	// AttestationCert can be null for Authenticate requests.
	pub attestation_cert: Option<Vec<u8>>,
}

pub fn parse_registration(app_id: String, client_data: Vec<u8>, registration_data: Vec<u8>) -> Result<Registration> {
    let reserved_byte = registration_data[0];
	if reserved_byte != 0x05 {
        return Err(U2fError::InvalidReservedByte);
	}
    
	let mut mem = Bytes::from(registration_data);
    
    //Start parsing ... advance the reserved byte.
    let _ = mem.split_to(1);

    // P-256 NIST elliptic curve
    let public_key = mem.split_to(65);

	let key_handle_size = mem.split_to(1);
    let key_len = BigEndian::read_uint(&key_handle_size[..], 1);
    let key_handle = mem.split_to(key_len as usize);

    let cert_len = asn_length(mem.clone()).unwrap();;
    let certificate = mem.split_to(cert_len as usize);
    let signature = mem;

    let app_id_hash = TOTPBuilder::new()
        .key(&app_id.into_bytes())
        .hash_function(HashFunction::Sha256)
        .finalize()
        .unwrap()
        .generate();        

    let client_data_hash = TOTPBuilder::new()
        .key(&client_data)
        .hash_function(HashFunction::Sha256)
        .finalize()
        .unwrap()
        .generate();  

    let mut sig_base = vec![];
    sig_base.put(reserved_byte);
    sig_base.put(app_id_hash);
    sig_base.put(client_data_hash);
    sig_base.put(key_handle.clone()); 
    sig_base.put(public_key.clone()); 

    let input_pub_key = Input::from(&public_key[..]);
    let input_sig = Input::from(&signature[..]);
    let input_sig_base = Input::from(&sig_base[..]);
	let ret = signature::verify(&signature::ECDSA_P256_SHA256_FIXED, input_pub_key, input_sig_base, input_sig)
        .map_err(|error::Unspecified| U2fError::BadSignature)?;

    let registration = Registration {
        key_handle: key_handle[..].to_vec(),
        pub_key: public_key[..].to_vec(), 
        attestation_cert: Some(certificate[..].to_vec()),
    };

	Ok(registration)
}

// Decode initial bytes of buffer as ASN and return the length of the encoded structure.
// http://en.wikipedia.org/wiki/X.690
fn asn_length(mem: Bytes) -> Result<usize> {
	let mut buf = Cursor::new(mem);

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