use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use std::env;

// VehicleInput struct for deserializing incoming JSON data for creating/updating vehicles
#[derive(Deserialize)]
struct VehicleInput {
    make: String,
    model: String,
    year: i32,
    vin: String,
}

// Vehicle struct representing a record in the vehicles table
#[derive(Serialize, FromRow)]
struct Vehicle {
    id: i32,
    make: String,
    model: String,
    year: i32,
    vin: String,
    created_at: String,
    updated_at: String,
}


#[tokio::main]
async fn main() {
    
    // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string()).parse::<u16>().expect("PORT must be a valid number");

    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");
}
