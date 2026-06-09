pub mod handler;

use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/global", get(handler::global_search))
        .layer(axum::middleware::from_fn(::server_auth::guest_auth_middleware))
        .with_state(pool);

    Router::new().merge(r)
}
