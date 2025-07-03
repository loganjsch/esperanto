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

pub async fn verify_nitro_handler_with_policy(
    body: Bytes
) -> Response {
    println!("Received {} bytes for attestation.", body.len());

    

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