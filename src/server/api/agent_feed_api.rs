use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::agent_feed::service::{AgentFeedService, AgentType, CardType, CardStatus, AgentFeedCard};

pub fn agent_feed_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/feed", get(list_pending_cards).post(create_card))
        .route("/api/v1/feed/:card_id/resolve", put(resolve_card))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct CreateCardRequest {
    tenant_id: Uuid,
    agent_type: AgentType,
    card_type: CardType,
    title: String,
    description: String,
    proposed_action_payload: Option<serde_json::Value>,
}

async fn create_card(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCardRequest>,
) -> Result<Json<AgentFeedCard>, (StatusCode, String)> {
    let service = AgentFeedService::new(pool);
    let card = service
        .create_card(
            payload.tenant_id,
            payload.agent_type,
            payload.card_type,
            payload.title,
            payload.description,
            payload.proposed_action_payload,
        )
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(card))
}

#[derive(Deserialize)]
pub struct ListCardsQuery {
    tenant_id: Uuid,
}

async fn list_pending_cards(
    State(pool): State<PgPool>,
    axum::extract::Query(query): axum::extract::Query<ListCardsQuery>,
) -> Result<Json<Vec<AgentFeedCard>>, (StatusCode, String)> {
    let service = AgentFeedService::new(pool);
    let cards = service
        .list_pending_cards(query.tenant_id)
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(cards))
}

#[derive(Deserialize)]
pub struct ResolveCardRequest {
    tenant_id: Uuid,
    status: CardStatus,
}

async fn resolve_card(
    State(pool): State<PgPool>,
    Path(card_id): Path<Uuid>,
    Json(payload): Json<ResolveCardRequest>,
) -> Result<Json<AgentFeedCard>, (StatusCode, String)> {
    let service = AgentFeedService::new(pool);
    let card = service
        .resolve_card(payload.tenant_id, card_id, payload.status)
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(card))
}
