use axum::{routing::{get, post}, Router};

pub mod routes;

pub fn router(db: sqlx::PgPool) -> Router {
    Router::new()
        .route("/", get(routes::list_incidents).post(routes::create_incident))
        .route("/:id/approve", post(routes::approve_incident))
        .with_state(db)
}
