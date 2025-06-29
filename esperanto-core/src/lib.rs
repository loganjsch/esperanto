// Main entry point for the Esperanto Core library
// Declares public modufles so esperanto-server can use them
pub mod policy;
pub mod verifier;
pub mod attestation;
pub mod error;

use tokio;
use crate::{
    attestation::nitro::verify_nitro_attestation_against_policy,
    policy::{AttestationPolicy, PcrMeasurement},
    error::CoreError,
};
use std::fs;



#[cfg(test)]
mod tests {
    use super::*;

    // tests for verify_nitro_attestation_against_policy 

    #[tokio::test]
    async fn test_valid_attestation() {
        
        // Load saved valid attestation doc from enclave
        let attestation_bytes = fs::read("tests/fixtures/payload_dump.bin")
            .expect("could not read attestation file");

        
        // --- Define a sample policy for testing ---
        // In the real system, this policy would be dynamically fetched from
        // policy management database based on an identifier (e.g., from the request context).
        // That means REMOVE .to_string() WHEN PARSING FROM CONFIG. we use to_string since we are building strings now
        let sample_policy = AttestationPolicy {
            id: "prod-analytics-enclave-v1".to_string(),
            name: "Production Analytics Enclave Policy V1".to_string(),
            description: "Policy for the main analytics processing enclave.".to_string(),
            platform_type: "aws_nitro_enclave".to_string(),
            expected_pcrs: vec![
                PcrMeasurement { index: 0, value: "820fd50e7c7150f49be69ac8b59f8caa067aee62c6771b85da1d1656eedfc9c7baa1a322a8707a86e6e8a00d60abf635".into() },
                PcrMeasurement { index: 1, value: "4b4d5b3661b3efc12920900c80e126e4ce783c522de6c02a2a5bf7af3a2b9327b86776f188e4be1c1c404a129dbda493".into() },
                PcrMeasurement { index: 2, value: "89d0a98c9b0fb130f10cda3dc0dc30cae78987cdb42a230446ebbd35add444b404977f0912e628b4b848cadbc7dfb8dc".into() },
                PcrMeasurement { index: 8, value: "ffae376606daf9066c26510d743bd15ab80a5f80afab438020238e9de1a8deb73c903c77ba47421ad2f4af20fd8c1b22".into() }
            ],
            // Some wraps object into an option
            expected_public_key: Some("my super secret key".to_string()),
            expected_user_data: Some("hello, world!".to_string()),
            nonce_required: false
        };

        // --- Generate a runtime nonce for testing (verifier would send this) ---
        // In a real scenario, the 'verifier' requesting attestation would generate and send this nonce
        // in its request, and the enclave would include it in its attestation document.
        let runtime_nonce_bytes = b"my_secret_runtime_nonce_1234567890abcdef";

        let result = verify_nitro_attestation_against_policy(
            &attestation_bytes,
            &sample_policy,
            None,
        );

        assert!(result.is_ok(), "Expected attestation to pass, got error: {:?}", result);
    }

    #[tokio::test]
    async fn test_invalid_attestation_platform_type() {
        
        // Load saved valid attestation doc from enclave
        let attestation_bytes = fs::read("tests/fixtures/payload_dump.bin")
            .expect("could not read attestation file");

        
        // --- Define a sample policy for testing ---
        // In the real system, this policy would be dynamically fetched from
        // policy management database based on an identifier (e.g., from the request context).
        // That means REMOVE .to_string() WHEN PARSING FROM CONFIG. we use to_string since we are building strings now
        let sample_policy = AttestationPolicy {
            id: "prod-analytics-enclave-v1".to_string(),
            name: "Production Analytics Enclave Policy V1".to_string(),
            description: "Policy for the main analytics processing enclave.".to_string(),
            platform_type: "bad_name".to_string(),
            expected_pcrs: vec![
                PcrMeasurement { index: 0, value: "820fd50e7c7150f49be69ac8b59f8caa067aee62c6771b85da1d1656eedfc9c7baa1a322a8707a86e6e8a00d60abf635".into() },
                PcrMeasurement { index: 1, value: "4b4d5b3661b3efc12920900c80e126e4ce783c522de6c02a2a5bf7af3a2b9327b86776f188e4be1c1c404a129dbda493".into() },
                PcrMeasurement { index: 2, value: "89d0a98c9b0fb130f10cda3dc0dc30cae78987cdb42a230446ebbd35add444b404977f0912e628b4b848cadbc7dfb8dc".into() },
                PcrMeasurement { index: 8, value: "ffae376606daf9066c26510d743bd15ab80a5f80afab438020238e9de1a8deb73c903c77ba47421ad2f4af20fd8c1b22".into() }
            ],
            // Some wraps object into an option
            expected_public_key: Some("my super secret key".to_string()),
            expected_user_data: Some("hello, world!".to_string()),
            nonce_required: false
        };

        // --- Generate a runtime nonce for testing (verifier would send this) ---
        // In a real scenario, the 'verifier' requesting attestation would generate and send this nonce
        // in its request, and the enclave would include it in its attestation document.
        let runtime_nonce_bytes = b"my_secret_runtime_nonce_1234567890abcdef";

        let result = verify_nitro_attestation_against_policy(
            &attestation_bytes,
            &sample_policy,
            None,
        );

        assert!(
            matches!(result, Err(CoreError::PolicyPlatformMismatch)),
            "Expected PolicyPlatformMismatch error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_valid_attestation_pcr_measurements() {
        
        // Load saved valid attestation doc from enclave
        let attestation_bytes = fs::read("tests/fixtures/payload_dump.bin")
            .expect("could not read attestation file");

        
        // --- Define a sample policy for testing ---
        // In the real system, this policy would be dynamically fetched from
        // policy management database based on an identifier (e.g., from the request context).
        // That means REMOVE .to_string() WHEN PARSING FROM CONFIG. we use to_string since we are building strings now
        let sample_policy = AttestationPolicy {
            id: "prod-analytics-enclave-v1".to_string(),
            name: "Production Analytics Enclave Policy V1".to_string(),
            description: "Policy for the main analytics processing enclave.".to_string(),
            platform_type: "aws_nitro_enclave".to_string(),
            expected_pcrs: vec![
                PcrMeasurement { index: 0, value: "820fd50e7c7150f49be69ac8b59f8caa067aee62c6771b85da1d1656eedfc9c7baa1a322a8707a86e6e8a00d60abf635".into() },
                PcrMeasurement { index: 1, value: "4b4d5b3661b3efc12920900c80e126e4ce783c522de6c02a2a5bf7af3a2b9327b86776f188e4be1c1c404a129dbda493".into() },
                PcrMeasurement { index: 2, value: "totallywrongpcrthisaintevenapcr".into() },
                PcrMeasurement { index: 8, value: "ffae376606daf9066c26510d743bd15ab80a5f80afab438020238e9de1a8deb73c903c77ba47421ad2f4af20fd8c1b22".into() }
            ],
            // Some wraps object into an option
            expected_public_key: Some("my super secret key".to_string()),
            expected_user_data: Some("hello, world!".to_string()),
            nonce_required: false
        };

        // --- Generate a runtime nonce for testing (verifier would send this) ---
        // In a real scenario, the 'verifier' requesting attestation would generate and send this nonce
        // in its request, and the enclave would include it in its attestation document.
        let runtime_nonce_bytes = b"my_secret_runtime_nonce_1234567890abcdef";

        let result = verify_nitro_attestation_against_policy(
            &attestation_bytes,
            &sample_policy,
            None,
        );

        assert!(
            matches!(result, Err(CoreError::PcrMismatch(_))),
            "Expected PcrMismatch error, got: {:?}",
            result
        );
    }

}
