use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use crate::server::domain::triage_feed::{self, TriageFeedItem};

pub fn router() -> Router {
    Router::new()
        .route("/api/triage/feed/:tenant_id", get(get_feed))
        .route("/api/triage/approve/:draft_id", post(approve_draft))
}

async fn get_feed(
    Path(tenant_id): Path<String>,
) -> Json<Vec<TriageFeedItem>> {
    let feed = triage_feed::get_triage_feed(&tenant_id);
    Json(feed)
}

async fn approve_draft(
    Path(draft_id): Path<String>,
) -> Result<Json<()>, String> {
    triage_feed::approve_action_draft(&draft_id)?;
    Ok(Json(()))
}
