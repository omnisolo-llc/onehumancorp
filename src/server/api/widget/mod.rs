pub mod chat;

use crate::db::DB;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderValue, header::CACHE_CONTROL},
    response::Response,
    routing::{get, post},
};
use std::sync::Arc;

pub fn router(db: Arc<DB>, auth: Arc<::server_auth::Store>) -> Router {
    let state = Arc::new(chat::WidgetChatState::new(db));

    Router::new()
        .route("/conversations", post(chat::create_conversation))
        .route("/messages", post(chat::create_message))
        .route(
            "/{tenant_id}/conversations/{conversation_id}/messages",
            get(chat::get_messages),
        )
        .with_state(state)
        .route_layer(axum::middleware::from_fn_with_state(
            auth,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .layer(DefaultBodyLimit::max(65_536))
        .layer(axum::middleware::map_response(private_response))
}

// This wraps authentication and extractor rejections as well as successful reads.
async fn private_response(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("private, no-store"));
    response
}
