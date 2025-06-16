// Application entry point for starting the Esperanto server

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