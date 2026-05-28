pub mod handlers;
#[cfg(test)]
pub mod tests;

use axum::{
    routing::{post, put},
    Router,
};
use std::sync::Arc;
use crate::hub::Hub;

pub fn router(hub: Arc<Hub>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let unauthenticated_router = Router::new()
        .route("/stops/:stop_id", put(handlers::update_stop_status_handler));

    let authenticated_router = Router::new()
        .route("/batches", post(handlers::create_batch_handler))
        .route("/batches/:id/dispatch", post(handlers::generate_driver_session_handler))
        .layer(axum::middleware::from_fn({
            move |req, next| {
                let hub = hub.clone();
                async move {
                    use axum::response::IntoResponse;
                    let store = std::sync::Arc::new(crate::auth::Store::new());
                    let auth_header = req.headers().get("authorization").and_then(|h| h.to_str().ok());
                    let token = match auth_header {
                        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
                        _ => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                    };
                    let claims = match store.validate_token(token).await {
                        Ok(c) => c,
                        Err(_) => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                    };
                    let mut req = req;
                    req.extensions_mut().insert(claims);
                    next.run(req).await
                }
            }
        }));

    Router::new()
        .merge(unauthenticated_router)
        .merge(authenticated_router)
}
