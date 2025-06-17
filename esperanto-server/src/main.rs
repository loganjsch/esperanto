// Application entry point for starting the Esperanto server
// Will set up logging, create API router by calling the api module, 
// binds to a TCP listener, and starts serving requests.

mod api; // Declare the api module

#[tokio::main]
async fn main() {
    // Setup logging/tracing
    // ...

    let app = api::router::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}