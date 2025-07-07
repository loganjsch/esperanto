## Overview

These policies will be stored somehwere else and loaded and consumed by the core, not stored in /policies in the core. For development purposes though, I have two sample policies chillin here. 

## Policy Types
Policies have two types: 

### Access Policies (access_policy)

These policies define what can be done with trusted resources. These are standard SARC (Subject Action Resource Context).

General layout: 
```yaml
# access_policy_pii_data_transfer.yaml
policyName: "AllowPIITransferToTrustedEnclave"
description: "Authorizes PII data transfer only from trusted TPM nodes to trusted Nitro Enclaves."
appliesToResourceType: "data_transfer" # A custom resource type representing data movement

rules:
  - name: "AllowSecurePIIFlow"
    effect: "ALLOW"
    conditions:
      logicalOperator: "AND"
      conditions:
        # 1. Source Asset (TPM Node) must be TRUSTED
        - attribute: "request.source.asset.type"
            operator: "equals"
            value: "tpm_node"
        - attribute: "request.source.asset.trust_status"
            operator: "equals"
            value: "TRUSTED"
        - ....

        # 2. Destination Asset (Nitro Enclave) must be TRUSTED
        - attribute: "request.destination.asset.type"
          operator: "equals"
          value: "aws_nitro_enclave"
        - ....

        # 3. Data Classification and Action
        - attribute: "request.data.classification"
          operator: "equals"
          value: "PII" # Only allow data explicitly classified as PII under this rule
        - ....


  - name: "DenyUntrustedOrNonPIIFlow"
    effect: "DENY"
    # This rule acts as a catch-all for anything not explicitly allowed by the "AllowSecurePIIFlow" rule.
    # It reinforces the Zero Trust principle of "default deny".
    conditions:
      logicalOperator: "ALWAYS_TRUE"
      value: "true" # This rule always applies if no preceding ALLOW rule matched

defaultEffect: "DENY" # Default to deny all requests not explicitly allowed
```

### Hardware Trust Policies (integrity_policy)

These policies define what is considered a ***trusted resource.***

These policies are platform specific, and will be based on the content provided in its attestation and the scope of security it guarantees. They reference the 'golden values' collected at enrollment time for a given resource.

Here is a general layout: 
```yaml
# policy_tpm_linux_server_integrity.yaml
policyName: "LinuxServerBootIntegrity_Prod_v1.1"
description: "Defines the trusted boot state for production Linux servers using TPM 2.0."
appliesToAssetType: "tpm_node" # This policy applies to assets identified as TPM-enabled nodes

goldenMeasurements:
  - name: "golden_pcr0_bios_firmware"
    type: "pcr_sha256"
    expectedHashRef: "vault://goldenvault/tpm/linux_prod_v1.1/pcr0_sha256"
  - ....
  - name: "allowed_parent_iam_roles"
    type: "regex_list"
    expectedValue:
  - ....
  

evaluationRules:
  logicalOperator: "AND" # All conditions must be true for the node to be TRUSTED
  conditions:
    - attribute: "verifier.result.pcr0_sha256_match"
      operator: "equals"
      value: "MATCHED"
    - ....

defaultTrustStatus: "UNKNOWN" # If any rule fails or no attestation is available, default to UNKNOWN
```


## Parsing Workflow

| File           | Role                               | Workflow                                                                                              |
| :------------- | :--------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `definitions.rs` | **Schema Definition** | Defines the Rust structs and enums that blueprint the structure of all policies.                   |
| `parser.rs`      | **Structural Validation** | Takes raw policy YAML, deserializes it into the defined structs, and performs structural/schema validation. |
| `resolver.rs`    | **Reference Resolution (Hydration)** | Orchestrates the entire loading process: it fetches the raw policy, calls `parser.rs` to parse and validate it, and then fetches any external data (e.g., golden values from a vault) to fully complete the policy.  |

Returns appriate policy struct for trust engine to evaluate presented attestation doc against. Woohoo!

In the future, a policyManager will orchestrate the use of multiple policies etc... specifically, the request coming in by the PEP will say this is the target resource, (as deifned by authorization request api?) and the policyManager will decide what policies to select for eval.

but for this MVP, request coming in specifies the policies that need to be loading - the handler will just call the resolver on x, y, z polcies and thats that. then the resolver, instead of sending back to the policy manager for it to send it to the trust engine, will just send the validated, resolved policy struct to the nitro engine.