// Conatins logic for loading and parsing the policies.yml file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


// A single PCR measurement (index and its hex value)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcrMeasurement {
    pub index: u8,
    pub value: String, // Hex-encoded hash
}

// Attestation Policy structure, defining the "golden values".
// It specifies what a trusted state looks like for a particular entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttestationPolicy {
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

// A fucntion to retrieve policies 