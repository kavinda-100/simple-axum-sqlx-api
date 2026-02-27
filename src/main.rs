// imports
use axum::{
    Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post}
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use dotenv::dotenv;
use std::env;
use chrono::NaiveDateTime;

// modules
mod utils;
use utils::response::ApiResponse;

// VehicleInput struct for deserializing incoming JSON data for creating/updating vehicles
#[derive(Deserialize, Debug)]
struct VehiclePayload {
    make: String,
    model: String,
    year: i32,
    vin: String,
}

// Vehicle struct representing a record in the vehicles table
#[derive(Serialize, FromRow, Debug)]
struct Vehicle {
    id: i32,
    make: String,
    model: String,
    year: i32,
    vin: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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

    // Run database migrations to ensure the vehicles table exists
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/vehicles", get(get_all_vehicles))
        .route("/vehicles", post(create_vehicle))
        .route("/vehicles/{id}", get(get_vehicle_by_id))
        .route("/vehicles/{id}", post(update_vehicle))
        .route("/vehicles/{id}", axum::routing::delete(delete_vehicle))
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
async fn get_all_vehicles(State(pool): State<PgPool>) -> Result<Json<ApiResponse<Vec<Vehicle>>>, StatusCode> {
    // for debugging purposes, log that we're fetching vehicles
    println!("Fetching all vehicles from the database...");

    // Fetch all vehicles from the database and return them as JSON
    let vehicles = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::new(true, StatusCode::OK, "Successfully fetched all vehicles".to_string(), Some(vehicles))))

}

/**
 * Handler for creating a new vehicle, accepts JSON input and inserts a new record into the database.
 */
async fn create_vehicle(State(pool): State<PgPool>, Json(payload): Json<VehiclePayload>) -> Result<Json<ApiResponse<Vehicle>>, StatusCode> {

    // Log the incoming payload for debugging purposes
    println!("Received payload for creating vehicle: {:?}", payload);

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

    Ok(Json(ApiResponse::new(true, StatusCode::CREATED, "Vehicle created successfully".to_string(), Some(vehicle))))
}

/**
 * Handler for get a single vehicle by ID, accepts a path parameter and returns the corresponding vehicle record.
 */
async fn get_vehicle_by_id(State(pool): State<PgPool>, axum::extract::Path(id): axum::extract::Path<i32>) -> Result<Json<ApiResponse<Vehicle>>, StatusCode> {
    // Fetch the vehicle with the specified ID from the database
    let vehicle = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse::new(true, StatusCode::OK, "Vehicle found".to_string(), Some(vehicle))))
}

/**
 * Handler for updating an existing vehicle, accepts a path parameter for the vehicle ID and JSON input for the updated data.
 */
async fn update_vehicle(State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload): Json<VehiclePayload>) -> Result<Json<ApiResponse<Vehicle>>, StatusCode> {
    // Update the vehicle with the specified ID in the database and return the updated record
    let vehicle = sqlx::query_as::<_, Vehicle>(
        "UPDATE vehicles SET make = $1, model = $2, year = $3, vin = $4, updated_at = CURRENT_TIMESTAMP WHERE id = $5 RETURNING *"
    )
    .bind(&payload.make)
    .bind(&payload.model)
    .bind(payload.year)
    .bind(&payload.vin)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse::new(true, StatusCode::OK, "Vehicle updated successfully".to_string(), Some(vehicle))))
}

/**
 * Handler for deleting a vehicle, accepts a path parameter for the vehicle ID and deletes the corresponding record from the database.
 */
async fn delete_vehicle(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Delete the vehicle with the specified ID from the database
    let result = sqlx::query("DELETE FROM vehicles WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }   
    else {
        return Ok(Json(ApiResponse::new(true, StatusCode::NO_CONTENT, "Vehicle deleted successfully".to_string(), None)));
    }
}