use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::controllers::vehicle_controller::{
    root_handler,
    get_all_vehicles,
    create_vehicle,
    get_vehicle_by_id,
    update_vehicle,
    delete_vehicle,
};

/// Creates and returns the vehicle routes
pub fn vehicle_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/vehicles", get(get_all_vehicles))
        .route("/vehicles", post(create_vehicle))
        .route("/vehicles/:id", get(get_vehicle_by_id))
        .route("/vehicles/:id", post(update_vehicle))
        .route("/vehicles/:id", axum::routing::delete(delete_vehicle))
        .with_state(pool)
}
