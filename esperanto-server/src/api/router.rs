// Define all API routes
// Will use axum router to map HTTP requests to corresponding fucntion



use axum::{
    routing::{get, post},
    Router,
};


// use crate::api::handlers::{verify_nitro_handler};