// Conatins logic for loading and parsing the policies.yml file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;



// Common metadata for any hardware trust policy
pub struct HardwareTrustPolicy {
    pub id: String,         // Unique identifier for this policy (e.g., "LinuxServerBootIntegrity_v1.1")
    pub name: String,
    pub description: String,
    pub default_trust_status: String, // e.g., "UNKNOWN", "UNTRUSTED"
    pub applies_to_asset_type: String, // e.g., "tpm_node", "aws_nitro_enclave"

    // This is where the magic happens: an enum holding the specific details
    pub specific_policy_details: SpecificPolicyDetails,
}

// An enum that branches based on the asset type
// enum variants with associated data : VariantName(AssociatedData)
pub enum SpecificPolicyDetails {
    AwsNitroEnclave(NitroEnclavePolicyConfig),
    // TpmNodePolicy(TpmNodePolicyConfig),
    // GoogleConfidentialVm(GoogleVmPolicyConfig), // For future expansion
    // Add other types as you expand support
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

// pub struct TpmNodePolicyConfig {}
// pub struct GoogleVmPolicyConfig {}

// A single PCR measurement (index and its hex value)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcrMeasurement {
    pub index: u8,
    pub value: String, // Hex-encoded hash
}

// why did I make this? 
#[derive(Debug, Deserialize)]
pub struct GoldenValues {
    pub pcrs: HashMap<String, String>,
    // other values like min_cpu...
}

// A function to load a specific policy from the file
pub fn load_policy(_policy_id: &str) -> Result<AttestationPolicy, crate::error::CoreError> {
    // ... logic to open policies.yml, find the policy, and parse it ...
    unimplemented!()
}

// A function to parse policies

// A fucntion to retrieve policies maybe 