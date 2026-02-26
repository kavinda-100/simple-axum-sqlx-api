use axum::{
    Json, Router, extract::State, http::StatusCode, routing::{get, post}
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use dotenv::dotenv;
use std::env;

// VehicleInput struct for deserializing incoming JSON data for creating/updating vehicles
#[derive(Deserialize)]
struct VehiclePayload {
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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string()).parse::<u16>().expect("PORT must be a valid number");

    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/vehicles", get(get_all_vehicles))
        .route("/vehicles", post(create_vehicle))
        .with_state(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Server running on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await.unwrap();
}

/**
 * Root handler for the base route, returns a welcome message.
 */
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Welcome to the Vehicle API!"
    }))
}

/**
 * Handler for getting all vehicles, returns a list of all vehicles in the database.
 */
async fn get_all_vehicles(State(pool): State<PgPool>) -> Result<Json<Vec<Vehicle>>, StatusCode> {
    let vehicles = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vehicles))
}

/**
 * Handler for creating a new vehicle, accepts JSON input and inserts a new record into the database.
 */
async fn create_vehicle(State(pool): State<PgPool>, Json(payload): Json<VehiclePayload>) -> Result<Json<Vehicle>, StatusCode> {

    // Insert the new vehicle into the database and return the created record
    let vehicle = sqlx::query_as::<_, Vehicle>(
        "INSERT INTO vehicles (make, model, year, vin) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.make)
    .bind(&payload.model)
    .bind(payload.year)
    .bind(&payload.vin)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;   

    Ok(Json(vehicle))
}