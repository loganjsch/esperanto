// Define all API routes
// Will use axum router to map HTTP requests to corresponding fucntion
use axum::{Router, routing::post};
use crate::api::handlers;

pub fn create_router() -> Router {
    Router::new()
        // Add a route for verifying Nitro attestation
        .route("/verify/nitro_enclave", post(handlers::verify_nitro_handler))
        // Add more routes as needed
}
