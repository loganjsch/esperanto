// Defines all errors core logic can produce. 

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Policy with ID '{0}' not found")]
    PolicyNotFound(String),

    #[error("Attestation signature is invalid")]
    SignatureInvalid,

    #[error("PCR values do not match the policy")]
    PcrMismatch,

    #[error("Failed to parse attestation document")]
    ParseError,
}