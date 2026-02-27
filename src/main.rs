use dotenv::dotenv;
use std::env;

// Module declarations
mod controllers;
mod db;
mod models;
mod routes;
mod utils;

use routes::api::create_routes;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Establish database connection and run migrations
    let pool = db::establish_connection().await;

    // Build the application with all routes
    let app = create_routes(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:{}", port);
    println!("API v1 available at http://0.0.0.0:{}/api/v1", port);
    axum::serve(listener, app).await.unwrap();
}
