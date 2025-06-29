// Bridge to translate HTTP requests into calls to the core Esperanto Library
use axum::{Json, http::StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::json;
use esperanto_core::{
    policy::{AttestationPolicy, PcrMeasurement},
    attestation::nitro::verify_nitro_attestation_against_policy
}; 
use axum::body::Bytes;
use std::fs::File;
use std::io::Write;

#[derive(Deserialize)]
pub struct PlaceholderRequest {}

// the parameters of this function are an extractor
// extractors tells axum to try to parse body of http request into placeholderrequest struct

pub async fn verify_nitro_handler(
    body: Bytes
) -> Response {
    println!("Received {} bytes", body.len());

    // Save to file for development use
    match File::create("payload_dump.bin") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&body) {
                eprintln!("Failed to write to file: {}", e);
            } else {
                println!("Saved request body to payload_dump.bin");
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }

    let response_body = json!({
        "status" : "success",
        "message" : "Endpoint hit and body saved."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(response_body)).into_response()
}

// This would typically be loaded once at application startup and passed around
// Fsor a quick test, we can embed in here 
// static mut AWS_NITRO_ROOT_CA_DER: &str = r#"-----BEGIN CERTIFICATE-----
// MIICETCCAZagAwIBAgIRAPkxdWgbkK/hHUbMtOTn+FYwCgYIKoZIzj0EAwMwSTEL
// MAkGA1UEBhMCVVMxDzANBgNVBAoMBkFtYXpvbjEMMAoGA1UECwwDQVdTMRswGQYD
// VQQDDBJhd3Mubml0cm8tZW5jbGF2ZXMwHhcNMTkxMDI4MTMyODA1WhcNNDkxMDI4
// MTQyODA1WjBJMQswCQYDVQQGEwJVUzEPMA0GA1UECgwGQW1hem9uMQwwCgYDVQQL
// DANBV1MxGzAZBgNVBAMMEmF3cy5uaXRyby1lbmNsYXZlczB2MBAGByqGSM49AgEG
// BSuBBAAiA2IABPwCVOumCMHzaHDimtqQvkY4MpJzbolL//Zy2YlES1BR5TSksfbb
// 48C8WBoyt7F2Bw7eEtaaP+ohG2bnUs990d0JX28TcPQXCEPZ3BABIeTPYwEoCWZE
// h8l5YoQwTcU/9KNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUkCW1DdkF
// R+eWw5b6cp3PmanfS5YwDgYDVR0PAQH/BAQDAgGGMAoGCCqGSM49BAMDA2kAMGYC
// MQCjfy+Rocm9Xue4YnwWmNJVA44fA0P5W2OpYow9OYCVRaEevL8uO1XYru5xtMPW
// rfMCMQCi85sWBbJwKKXdS6BptQFuZbT73o/gBh1qUxl/nNr12UO8Yfwr6wPLb+6N
// IwLz3/Y=
// -----END CERTIFICATE-----"#

// In a real application, you'd load this securely once.
// This is a simplified example.
// async fn initialize_root_ca() -> Result<(), String> {
//     unsafe {
//         if AWS_NITRO_ROOT_CA_DER.is_none() {
//             // IMPORTANT: Replace "path/to/AWS_NitroEnclaves_Root-G1.pem" with the actual path
//             // to your downloaded and verified AWS Nitro Root CA certificate.
//             // This path must be accessible to your running server.
//             match load_aws_nitro_root_ca("../../../dev-resources/root.pem") {
//                 Ok(der_bytes) => {
//                     AWS_NITRO_ROOT_CA_DER = Some(der_bytes);
//                     println!("AWS Nitro Root CA loaded successfully.");
//                 }
//                 Err(e) => {
//                     return Err(format!("Failed to load AWS Nitro Root CA: {:?}", e));
//                 }
//             }
//         }
//     }
//     Ok(())
// }


pub async fn verify_nitro_handler_with_policy(
    body: Bytes
) -> Response {
    println!("Received {} bytes for attestation.", body.len());

    // // Ensure CA is loaded (in real system, this would be part of app state or a global setup)
    // if unsafe { AWS_NITRO_ROOT_CA_DER.is_none() } {
    //     if let Err(e) = initialize_root_ca().await {
    //         eprintln!("Initialization error: {}", e);
    //         let response_body = json!({
    //             "status": "error",
    //             "message": e
    //         });
    //         return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_body)).into_response();
    //     }
    // }
    // let aws_root_ca_der = unsafe { AWS_NITRO_ROOT_CA_DER.as_ref().unwrap() };


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
            PcrMeasurement { index: 0, value: "479f7e78fb778c9dc9bea40b239218444b70a19ce58759732ffc6e97902e7e022968401f27a90cc159748052f9bdd2a6".into() },
            PcrMeasurement { index: 1, value: "4b4d5b3661b3efc12920900c80e126e4ce783c522de6c02a2a5bf7af3a2b9327b86776f188e4be1c1c404a129dbda493".into() },
            PcrMeasurement { index: 2, value: "b3791f9618cfa1f97962877290b3d87630abc2e84785cb05d2cff1ed06d753018fef5536e8f3c3977ea468dfc5deb153".into() }
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

    match verify_nitro_attestation_against_policy(
        &body,
        &sample_policy,
        Some(runtime_nonce_bytes),
    ) {
        Ok(claims) => {
            println!("Attestation Document Verified Successfully and Policy Matched!");
            println!("Extracted Claims: {:?}", claims);

            let response_body = json!({
                "status": "success",
                "message": "Nitro Enclave attested and policy matched.",
                "claims": {
                    "pcrs": claims.pcr_measurements,
                    "public_key": claims.public_key,
                    "user_data": claims.user_data,

                }
            });
            (StatusCode::OK, Json(response_body)).into_response()
        },
        Err(e) => {
            eprintln!("Attestation or Policy Verification Failed: {:?}", e);
            let response_body = json!({
                "status": "failure",
                "message": format!("Attestation or Policy verification failed: {:?}", e)
            });
            (StatusCode::BAD_REQUEST, Json(response_body)).into_response()
        }
    }
}




// pub struct VerifyNitroRequest {
//     #[serde(rename = "policyId")]
//     policy_id: String,
//     #[serde(rename = "attestationDocument")]
//     attestation_document: String, // Base64 encoded
// }

// pub async fn verify_nitro_handler(Json(payload): Json<VerifyNitroRequest>) -> Response {
//     // This is where the magic happens! We call the core logic.
//     let result = verifier::verify_nitro_attestation(
//         &payload.policy_id,
//         &payload.attestation_document
//     );

//     match result {
//         Ok(_) => (StatusCode::OK, Json("Verification successful")).into_response(),
//         Err(e) => {
//             // Map the core error to an HTTP status code
//             let status_code = match e {
//                 CoreError::PolicyNotFound(_) => StatusCode::NOT_FOUND,
//                 CoreError::SignatureInvalid => StatusCode::BAD_REQUEST,
//                 CoreError::PcrMismatch => StatusCode::UNPROCESSABLE_ENTITY,
//                 CoreError::ParseError => StatusCode::BAD_REQUEST,
//             };
//             (status_code, Json(e.to_string())).into_response()
//         }
//     }
// }