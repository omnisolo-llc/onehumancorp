pub mod incident_routes;

use axum::{routing::{get, post}, Router};

pub fn router(db: sqlx::PgPool) -> Router {
    Router::new()
        .route("/", get(incident_routes::list_incidents).post(incident_routes::create_incident))
        .route("/:id/approve", post(incident_routes::approve_incident))
        .with_state(db)
}
