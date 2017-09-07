use std::error;
use std::fmt;

#[derive(Debug)]
pub enum U2fError {
    Asm1DecoderError,
    BadSignature,
    RandomSecureBytesError,
    InvalidReservedByte,
    ChallengeExpired
}

impl fmt::Display for U2fError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            U2fError::Asm1DecoderError => write!(f, "ASM1 Decoder error"),
            U2fError::BadSignature => write!(f, "Not able to verify signature"),
            U2fError::RandomSecureBytesError => write!(f, "Not able to generate random bytes"),
            U2fError::InvalidReservedByte => write!(f, "Invalid Reserved Byte"),
            U2fError::ChallengeExpired => write!(f, "ChallengeExpired"),
        }
    }
}

impl error::Error for U2fError {
    fn description(&self) -> &str {
        match *self {
            U2fError::Asm1DecoderError => "Error attempting to decode Asm1 message",
            U2fError::BadSignature => "Error attempting to verify provided signature",
            U2fError::RandomSecureBytesError => "Error attempting to generate random bytes",
            U2fError::InvalidReservedByte => "Error attempting to parse Reserved Byte",
            U2fError::ChallengeExpired => "Challenge has expired",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            U2fError::Asm1DecoderError => None,
            U2fError::BadSignature => None,
            U2fError::RandomSecureBytesError => None,
            U2fError::InvalidReservedByte => None,
            U2fError::ChallengeExpired => None,            
        }
    }
}