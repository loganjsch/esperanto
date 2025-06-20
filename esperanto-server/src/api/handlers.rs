// Bridge to translate HTTP requests into calls to the core Esperanto Library
use axum::{Json, http::StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::json;
use esperanto_core::{policy, verifier, error::CoreError}; // Use the library!

#[derive(Deserialize)]
pub struct PlaceholderRequest {}

// the parameters of this function are an extractor
// extractors tells axum to try to parse body of http request into placeholderrequest struct
pub async fn verify_nitro_handler(
    Json(_payload): Json<PlaceholderRequest>
) -> Response {
    // for testing, just prove the endpoint works
    // 'not implemented' == success
    let response_body = json!({
        "status" : "success",
        "message" : "Endpoint hit, logic not implemented yet."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(response_body)).into_response()
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