use server_lib::db::DB;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionCard {
    pub id: Uuid,
    pub tenant_id: String,
    pub agent_id: String,
    pub trigger_event: String,
    pub context_summary: String,
    pub proposed_action: serde_json::Value,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateActionCardRequest {
    pub tenant_id: String,
    pub agent_id: String,
    pub trigger_event: String,
    pub context_summary: String,
    pub proposed_action: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateActionCardStateRequest {
    pub state: String,
}

pub struct ActionFeedService {
    db: Arc<DB>,
}

impl ActionFeedService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn list_cards(&self, tenant_id: &str) -> Result<Vec<ActionCard>, String> {
        if self.db.is_sqlite() {
            return Err("ActionFeed not supported in SQLite".to_string());
        }

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            "SELECT id, tenant_id, agent_id, trigger_event, context_summary, proposed_action, state::text, created_at, updated_at FROM action_cards ORDER BY created_at DESC"
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let mut cards = Vec::new();
        for row in rows {
            cards.push(ActionCard {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                agent_id: row.get("agent_id"),
                trigger_event: row.get("trigger_event"),
                context_summary: row.get("context_summary"),
                proposed_action: row.get("proposed_action"),
                state: row.get("state"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        Ok(cards)
    }

    pub async fn create_card(&self, req: CreateActionCardRequest) -> Result<ActionCard, String> {
        if self.db.is_sqlite() {
            return Err("ActionFeed not supported in SQLite".to_string());
        }

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let row = sqlx::query(
            "INSERT INTO action_cards (tenant_id, agent_id, trigger_event, context_summary, proposed_action, state) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL') RETURNING id, tenant_id, agent_id, trigger_event, context_summary, proposed_action, state::text, created_at, updated_at"
        )
        .bind(&req.tenant_id)
        .bind(&req.agent_id)
        .bind(&req.trigger_event)
        .bind(&req.context_summary)
        .bind(&req.proposed_action)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(ActionCard {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            agent_id: row.get("agent_id"),
            trigger_event: row.get("trigger_event"),
            context_summary: row.get("context_summary"),
            proposed_action: row.get("proposed_action"),
            state: row.get("state"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_card_state(&self, tenant_id: &str, card_id: Uuid, new_state: &str) -> Result<ActionCard, String> {
         if self.db.is_sqlite() {
            return Err("ActionFeed not supported in SQLite".to_string());
        }

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // Note: casting to the custom enum type.
        let row = sqlx::query(
            "UPDATE action_cards SET state = $1::action_card_state, updated_at = NOW() WHERE id = $2 RETURNING id, tenant_id, agent_id, trigger_event, context_summary, proposed_action, state::text, created_at, updated_at"
        )
        .bind(new_state)
        .bind(card_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(ActionCard {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            agent_id: row.get("agent_id"),
            trigger_event: row.get("trigger_event"),
            context_summary: row.get("context_summary"),
            proposed_action: row.get("proposed_action"),
            state: row.get("state"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

// REST Handlers
async fn list_cards_handler(
    State(service): State<Arc<ActionFeedService>>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    match service.list_cards(&tenant_id).await {
        Ok(cards) => (StatusCode::OK, Json(cards)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn create_card_handler(
    State(service): State<Arc<ActionFeedService>>,
    Json(req): Json<CreateActionCardRequest>,
) -> impl IntoResponse {
    match service.create_card(req).await {
        Ok(card) => (StatusCode::CREATED, Json(card)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn update_card_state_handler(
    State(service): State<Arc<ActionFeedService>>,
    Path((tenant_id, card_id)): Path<(String, Uuid)>,
    Json(req): Json<UpdateActionCardStateRequest>,
) -> impl IntoResponse {
    match service.update_card_state(&tenant_id, card_id, &req.state).await {
        Ok(card) => (StatusCode::OK, Json(card)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub fn action_feed_routes(service: Arc<ActionFeedService>) -> Router {
    Router::new()
        .route("/api/v1/action-feed/:tenant_id/cards", get(list_cards_handler).post(create_card_handler))
        .route("/api/v1/action-feed/:tenant_id/cards/:card_id/state", put(update_card_state_handler))
        .with_state(service)
}
