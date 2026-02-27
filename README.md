# 🚗 Simple Axum SQLx API

A production-ready REST API built with Rust, Axum, and SQLx, featuring a clean architecture pattern similar to Express.js MVC structure. This project demonstrates best practices for building scalable, maintainable web services in Rust.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![SQLx](https://img.shields.io/badge/sqlx-0.8-green.svg)](https://github.com/launchbadge/sqlx)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## ✨ Features

- 🏗️ **Clean Architecture** - Organized into Models, Controllers, and Routes
- 🔄 **API Versioning** - Built-in support for API versioning (`/api/v1`)
- 📦 **Consistent Response Format** - Standardized JSON responses across all endpoints
- 🗃️ **Database Migrations** - Automatic schema migrations with SQLx
- ⚡ **Async/Await** - Built on Tokio for high-performance async operations
- 🔒 **Type Safety** - Leveraging Rust's type system for compile-time guarantees
- 📝 **Full CRUD** - Complete Create, Read, Update, Delete operations
- 🩺 **Health Check** - Built-in health monitoring endpoint

## 🛠️ Technologies Used

| Technology     | Purpose                                            |
| -------------- | -------------------------------------------------- |
| **Rust**       | Programming language                               |
| **Axum**       | Web framework built on Tokio                       |
| **SQLx**       | Async SQL toolkit with compile-time query checking |
| **Tokio**      | Async runtime                                      |
| **PostgreSQL** | Database                                           |
| **Serde**      | Serialization/deserialization                      |
| **Chrono**     | Date and time handling                             |
| **dotenv**     | Environment variable management                    |

## 📁 Project Structure

```
src/
├── main.rs                      # Application entry point
├── db/
│   └── mod.rs                   # Database connection & migrations
├── models/
│   ├── mod.rs
│   └── vehicle.rs               # Vehicle data models
├── controllers/
│   ├── mod.rs
│   ├── root_controller.rs       # Root & health endpoints
│   └── vehicle_controller.rs    # Vehicle business logic
├── routes/
│   ├── mod.rs
│   ├── api.rs                   # Main route composition
│   └── vehicle_routes.rs        # Vehicle route definitions
└── utils/
    ├── mod.rs
    └── response.rs              # Standard API response wrapper
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or higher
- PostgreSQL 12 or higher
- Cargo (comes with Rust)

### Installation

1. **Clone the repository**

    ```bash
    git clone https://github.com/kavinda-100/simple-axum-sqlx-api.git
    cd simple-axum-sqlx-api
    ```

2. **Set up PostgreSQL**

    ```bash
    # Create database
    createdb vehicles_db

    # Or using psql
    psql -U postgres
    CREATE DATABASE vehicles_db;
    ```

3. **Configure environment variables**

    Create a `.env` file in the project root:

    ```env
    DATABASE_URL=postgresql://postgres:your_password@localhost:5432/vehicles_db
    PORT=5000
    ```

4. **Install dependencies and run**

    ```bash
    # Build the project
    cargo build

    # Run the application
    cargo run
    ```

    The server will start on `http://0.0.0.0:5000`

### Database Migrations

Migrations run automatically on startup. The SQLx migration files are located in the `migrations/` directory:

```sql
-- migrations/0001_init_tables.sql
CREATE TABLE IF NOT EXISTS vehicles (
    id SERIAL PRIMARY KEY,
    make VARCHAR(255) NOT NULL,
    model VARCHAR(255) NOT NULL,
    year INT NOT NULL,
    vin VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 📚 API Documentation

### Base URL

```
http://localhost:5000
```

### Response Format

All API responses follow a consistent structure:

```json
{
	"success": true,
	"status_code": 200,
	"message": "Descriptive message",
	"data": {
		/* Response data */
	}
}
```

### Endpoints

#### Root Endpoints

| Method | Endpoint  | Description                  |
| ------ | --------- | ---------------------------- |
| GET    | `/`       | Welcome message and API info |
| GET    | `/health` | Health check endpoint        |

##### Example: Root Response

```http
GET /

Response:
{
  "message": "Welcome to the Vehicle API!",
  "version": "1.0",
  "endpoints": {
    "vehicles": "/api/v1/vehicles"
  }
}
```

---

#### Vehicle Endpoints

Base path: `/api/v1/vehicles`

##### 1. Get All Vehicles

```http
GET /api/v1/vehicles
```

**Response:** `200 OK`

```json
{
	"success": true,
	"status_code": 200,
	"message": "Successfully fetched all vehicles",
	"data": [
		{
			"id": 1,
			"make": "Toyota",
			"model": "Corolla",
			"year": 2020,
			"vin": "1HGBH41JXMN109186",
			"created_at": "2026-02-27T05:11:13.806911",
			"updated_at": "2026-02-27T05:11:13.806911"
		},
		{
			"id": 2,
			"make": "BMW",
			"model": "X5",
			"year": 2025,
			"vin": "5UXCR6C0XJLL12345",
			"created_at": "2026-02-27T05:14:36.899771",
			"updated_at": "2026-02-27T05:14:36.899771"
		}
	]
}
```

##### 2. Get Vehicle by ID

```http
GET /api/v1/vehicles/:id
```

**Response:** `200 OK`

```json
{
	"success": true,
	"status_code": 200,
	"message": "Vehicle found",
	"data": {
		"id": 1,
		"make": "Toyota",
		"model": "Corolla",
		"year": 2020,
		"vin": "1HGBH41JXMN109186",
		"created_at": "2026-02-27T05:11:13.806911",
		"updated_at": "2026-02-27T05:11:13.806911"
	}
}
```

**Error:** `404 Not Found`

```json
{
	"status_code": 404
}
```

##### 3. Create Vehicle

```http
POST /api/v1/vehicles
Content-Type: application/json

{
  "make": "Tesla",
  "model": "Model 3",
  "year": 2024,
  "vin": "5YJ3E1EA1KF123456"
}
```

**Response:** `201 Created`

```json
{
	"success": true,
	"status_code": 201,
	"message": "Vehicle created successfully",
	"data": {
		"id": 4,
		"make": "Tesla",
		"model": "Model 3",
		"year": 2024,
		"vin": "5YJ3E1EA1KF123456",
		"created_at": "2026-02-27T07:00:00.000000",
		"updated_at": "2026-02-27T07:00:00.000000"
	}
}
```

##### 4. Update Vehicle

```http
POST /api/v1/vehicles/:id
Content-Type: application/json

{
  "make": "Tesla",
  "model": "Model S",
  "year": 2024,
  "vin": "5YJ3E1EA1KF123456"
}
```

**Response:** `200 OK`

```json
{
	"success": true,
	"status_code": 200,
	"message": "Vehicle updated successfully",
	"data": {
		"id": 4,
		"make": "Tesla",
		"model": "Model S",
		"year": 2024,
		"vin": "5YJ3E1EA1KF123456",
		"created_at": "2026-02-27T07:00:00.000000",
		"updated_at": "2026-02-27T07:15:00.000000"
	}
}
```

##### 5. Delete Vehicle

```http
DELETE /api/v1/vehicles/:id
```

**Response:** `200 OK` (with JSON body)

```json
{
	"success": true,
	"status_code": 204,
	"message": "Vehicle deleted successfully",
	"data": null
}
```

**Error:** `404 Not Found`

```json
{
	"status_code": 404
}
```

### Status Codes

| Code | Description                 |
| ---- | --------------------------- |
| 200  | Success                     |
| 201  | Created                     |
| 204  | No Content (Delete success) |
| 404  | Not Found                   |
| 500  | Internal Server Error       |

## 🧪 Testing with API.http

The project includes an `API.http` file for testing with REST Client extensions:

```http
@baseUrl = http://localhost:5000
@apiV1 = http://localhost:5000/api/v1

### Get all vehicles
GET {{apiV1}}/vehicles

### Create a new vehicle
POST {{apiV1}}/vehicles
Content-Type: application/json

{
  "make": "Toyota",
  "model": "Camry",
  "year": 2024,
  "vin": "4T1B11HK1KU123456"
}
```

## 🏗️ Development Guide

### Adding a New Resource

To add a new resource (e.g., `users`), follow these steps:

1. **Define Database Schema** (create a new migration file in `migrations/`)

    ```sql
    -- migrations/0002_create_users_table.sql
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    ```

2. **Create Model** (`src/models/user.rs`)

    ```rust
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Deserialize, Debug)]
    pub struct UserPayload {
        pub name: String,
        pub email: String,
    }

    #[derive(Serialize, FromRow, Debug)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub email: String,
    }
    ```

3. **Create Controller** (`src/controllers/user_controller.rs`)

    ```rust
    use axum::{Json, extract::State, http::StatusCode};
    use sqlx::PgPool;
    use crate::models::user::{User, UserPayload};
    use crate::utils::response::ApiResponse;

    pub async fn get_all_users(
        State(pool): State<PgPool>
    ) -> Result<Json<ApiResponse<Vec<User>>>, StatusCode> {
        // Implementation
    }
    ```

4. **Create Routes** (`src/routes/user_routes.rs`)

    ```rust
    use axum::{Router, routing::get};
    use sqlx::PgPool;
    use crate::controllers::user_controller::get_all_users;

    pub fn user_routes(pool: PgPool) -> Router {
        Router::new()
            .route("/users", get(get_all_users))
            .with_state(pool)
    }
    ```

5. **Register Routes** (in `src/routes/api.rs`)
    ```rust
    fn api_v1_routes(pool: PgPool) -> Router {
        Router::new()
            .merge(vehicle_routes(pool.clone()))
            .merge(user_routes(pool.clone()))  // Add this
    }
    ```

### Database Queries

SQLx provides compile-time query checking. Example:

```rust
let vehicle = sqlx::query_as::<_, Vehicle>(
    "SELECT * FROM vehicles WHERE id = $1"
)
.bind(id)
.fetch_one(&pool)
.await?;
```

## 🔧 Configuration

Configuration is managed through environment variables:

| Variable       | Description                  | Default  |
| -------------- | ---------------------------- | -------- |
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `PORT`         | Server port                  | 5000     |

## 👨‍💻 Author

**Kavinda**

- GitHub: [@kavinda-100](https://github.com/kavinda-100)

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Excellent web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Powerful async SQL toolkit
- [Tokio](https://tokio.rs/) - The async runtime for Rust

---

⭐ If you find this project helpful, please consider giving it a star!
