-- CREATE DATABASE IF NOT EXISTS vehicles_db; -- Create the database if it doesn't exist

CREATE TABLE IF NOT EXISTS vehicles (
    id SERIAL PRIMARY KEY, -- Auto-incrementing primary key
    make VARCHAR(255) NOT NULL, -- Manufacturer of the vehicle
    model VARCHAR(255) NOT NULL, -- Model of the vehicle
    year INT NOT NULL, -- Year of manufacture
    vin VARCHAR(255) UNIQUE NOT NULL, -- Vehicle Identification Number, must be unique
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Timestamp of when the record was created
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Timestamp of when the record was last updated
);