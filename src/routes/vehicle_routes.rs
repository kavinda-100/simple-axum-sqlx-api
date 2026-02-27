use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::controllers::vehicle_controller::{
    create_vehicle, delete_vehicle, get_all_vehicles, get_vehicle_by_id, update_vehicle,
};

/// Creates and returns the vehicle routes
/// All routes here are prefixed with /vehicles (configured in api.rs)
pub fn vehicle_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/vehicles", get(get_all_vehicles).post(create_vehicle))
        .route(
            "/vehicles/{id}",
            get(get_vehicle_by_id)
                .put(update_vehicle)
                .delete(delete_vehicle),
        )
        .with_state(pool)
}
