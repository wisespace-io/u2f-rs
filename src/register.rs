
// Single enrolment or pairing between an application and a token.

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub key_handle: Vec<u8>,
	pub pub_key: String,

	// AttestationCert can be null for Authenticate requests.
	pub attestation_cert: Option<String>,
}
