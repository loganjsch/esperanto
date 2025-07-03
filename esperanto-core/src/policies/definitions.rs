// policies/definitions.rs

// Assume PcrMeasurement and TrustStatus are defined in common/types.rs
// use crate::common::types::{TrustStatus};

// Top-level struct for any hardware trust policy definition
pub struct HardwareTrustPolicyConfig {
    pub id: String,
    pub name: String,
    // ... common fields ...
    pub platform_attestation_params: PlatformAttestationParameters,
}

pub enum PlatformAttestationParameters {
    AwsNitroEnclave(NitroEnclavePolicyConfig),
    // TpmNode(TpmNodeAttestationConfig),
    // ...
}

// AWS Attestation Policy structure, defining the "golden values".
// It specifies what a trusted state looks like for a particular entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NitroEnclavePolicyConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub platform_type: String, // e.g., "aws_nitro_enclave"

    pub expected_pcrs: Vec<PcrMeasurement>,
    pub expected_public_key: Option<String>,
    pub expected_user_data: Option<String>,

    // Nonce handling policy (e.g., if a nonce is required to be present and match)
    // Note: The actual nonce value for verification is typically provided at runtime
    // by the relying party, not stored in the static policy.
    pub nonce_required: bool,

    // Add other policy rules as system evolves
}
// pub struct TpmNodeAttestationConfig {
//     pub expected_boot_pcrs: Vec<PcrMeasurement>,
//     // ... specific TPM fields ...
// }

// You'd also put your access policy definitions here
pub struct AttestationBasedAccessPolicyConfig {
    pub policy_name: String,
    // ... rules, effects ...
}