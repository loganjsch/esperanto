// esperanto-core/src/attestation/nitro.rs

//! AWS Nitro Enclave attestation verification using Evervault's crate (Apache 2.0).
//! This module parses and verifies attestation documents from Nitro Enclaves,
//! and checks them against a structured `AttestationPolicy`.

use std::collections::HashMap;
use attestation_doc_validation::{
    validate_and_parse_attestation_doc, 
    validate_expected_nonce, 
    validate_expected_pcrs, 
    PCRProvider
};
use x509_parser::pem::parse_x509_pem;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use hex;
use base64::engine::general_purpose::STANDARD as base64_standard;
use base64::Engine;

use crate::policy::AttestationPolicy;
use crate::error::CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcrMeasurement {
    pub index: u8,
    pub value: String,
}

#[derive(Debug)]
pub struct AttestationClaims {
    pub platform_type: String,
    pub pcr_measurements: HashMap<u8, String>,
    pub nonce: Option<String>,
    pub user_data: Option<String>,
    pub public_key: Option<String>

}

#[derive(Debug)]
pub enum AttestationVerificationError {
    Io(std::io::Error),
    CertificateParse(String),
    AttestationValidation(String),
    PolicyViolation(String),
    RequiredClaimMissing(String),
    InvalidInput(String),
}

impl From<std::io::Error> for AttestationVerificationError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}


/// Loads AWS Nitro Root CA from a PEM file and returns it as bytes.
pub fn load_nitro_root_ca_pem(path: &str) -> Result<Vec<u8>, AttestationVerificationError> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let (_, pem) = parse_x509_pem(&buf).map_err(|e| {
        AttestationVerificationError::CertificateParse(format!("Failed to parse PEM: {:?}", e))
    })?;
    Ok(pem.contents)
}

pub struct EsperantoPCRs {
    pcr_map: HashMap<u8, String>,
}

impl PCRProvider for EsperantoPCRs {
    fn pcr_0(&self) -> Option<&str> { self.pcr_map.get(&0).map(|s| s.as_str()) }
    fn pcr_1(&self) -> Option<&str> { self.pcr_map.get(&1).map(|s| s.as_str()) }
    fn pcr_2(&self) -> Option<&str> { self.pcr_map.get(&2).map(|s| s.as_str()) }
    fn pcr_8(&self) -> Option<&str> { self.pcr_map.get(&8).map(|s| s.as_str()) }
}

/// Verify AWS Nitro Enclave attestation document using only exposed APIs.
///
/// # Arguments
/// - `attestation_doc_bytes`: Raw COSE-signed attestation document bytes.
/// - `policy`: Expected attestation policy (PCRs, nonce required flag).
/// - `runtime_nonce`: Optional runtime nonce bytes for freshness check.
///
/// # Returns
/// `Ok(AttestationClaims)` if valid and matches policy, otherwise an error

pub fn verify_nitro_attestation_against_policy(
    attestation_doc_bytes: &[u8],
    policy: &AttestationPolicy,
    runtime_nonce: Option<&[u8]>,
) -> Result<AttestationClaims, CoreError> {
    if policy.platform_type != "aws_nitro_enclave" {
        return Err(CoreError::PolicyPlatformMismatch);
    }

    // Validate signature and parse attestation doc internally
    // returns type AttestResult<AttestationDoc>
    // inner error might expose evervault problem
    let doc = validate_and_parse_attestation_doc(attestation_doc_bytes)
        .map_err(|e| CoreError::AttestationValidationError(format!("{:?}", e)))?;

    for p in &policy.expected_pcrs {
        println!("From policy: PCR{} = {}", p.index, p.value);
    }

    // Build expected PCR map
    let expected_pcr_map: HashMap<u8, String> = policy.expected_pcrs.iter()
        .map(|p| (p.index, p.value.clone()))
        .collect();

    for (idx, val) in &expected_pcr_map {
        println!("Built expected_pcr_map: PCR{} = {}", idx, val);
    }

    let expected_pcrs = EsperantoPCRs{pcr_map: expected_pcr_map};

    // Validate expected PCRs using crate's exposed API
    validate_expected_pcrs(&doc, &expected_pcrs)
        .map_err(|e| CoreError::PcrMismatch(format!("{:?}", e)))?;

    // Validate nonce if required
    if policy.nonce_required {
        let expected_nonce_b64 = match runtime_nonce {
            Some(nonce_bytes) => base64_standard.encode(nonce_bytes),
            None => return Err(CoreError::MissingNonce),
        };        

        validate_expected_nonce(&doc, &expected_nonce_b64)
            .map_err(|e| CoreError::NonceInvalid(format!("{:?}", e)))?;
    }

    // Build PCR map from validated doc to return (using crate's public API)
    // This relies on doc.pcrs being accessible
    let mut pcr_map = HashMap::new();
    for (&idx, val) in doc.pcrs.iter() {
        pcr_map.insert(idx.try_into().unwrap(), hex::encode(val));
    }

    Ok(AttestationClaims {
        platform_type: "aws_nitro_enclave".to_string(),
        pcr_measurements: pcr_map,
        nonce: doc.nonce.as_ref().map(|n| base64_standard.encode(n)),
        user_data: doc.user_data.as_ref().map(hex::encode),
        public_key: doc.public_key.as_ref().map(hex::encode)
    })
}
