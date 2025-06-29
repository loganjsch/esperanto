// Defines all errors core logic can produce. 

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Policy with ID '{0}' not found")]
    PolicyNotFound(String),

    #[error("Platform for this policy does not match attestation platform.")]
    PolicyPlatformMismatch,

    #[error("Error while parsing and validating attestation doc: {0}")]
    AttestationValidationError(String),

    #[error("Attestation signature is invalid")]
    SignatureInvalid,

    #[error("PCR value doe not match the policy: {0}")]
    PcrMismatch(String),

    #[error("Nonce required by policy but not provided")]
    MissingNonce,

    #[error("Nonce mismatch: {0}")]
    NonceInvalid(String),

}