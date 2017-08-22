
// Single enrolment or pairing between an application and a token.

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    key_handle: Vec<u8>,
	pub_key: String,

	// AttestationCert can be null for Authenticate requests.
	attestation_cert: Option<String>,
}
