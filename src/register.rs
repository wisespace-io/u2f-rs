
// Single enrolment or pairing between an  application and a token.

pub struct Registration {
    key_handle: Vec<u8>,
	pub_key: String,

	// AttestationCert can be nil for Authenticate requests.
	attestation_cert: Option<String>,
}
