use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

// VehiclePayload struct for deserializing incoming JSON data for creating/updating vehicles
#[derive(Deserialize, Debug)]
pub struct VehiclePayload {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub vin: String,
}

// Vehicle struct representing a record in the vehicles table
#[derive(Serialize, FromRow, Debug)]
pub struct Vehicle {
    pub id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub vin: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
