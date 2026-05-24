use axum::{
    routing::post,
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub mod handler;

pub fn router(pool: PgPool, store: Arc<crate::auth::Store>) -> Router {
    let state = handler::SyncState { pool, store };
    Router::new()
        .route("/missions", post(handler::sync_missions_handler))
        .with_state(state)
}
