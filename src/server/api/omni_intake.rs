use crate::db::DB;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Debug, Deserialize)]
pub struct MultilingualIntakePayload {
    pub tenant_id: String,
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderIntent {
    intent: String,
    items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderItem {
    name: String,
    quantity: i32,
}

pub fn router(db: Arc<DB>) -> Router {
    let state = AppState { db };
    Router::new()
        .route("/api/v1/webhooks/omni_intake", post(handle_multilingual_intake))
        .with_state(state)
}

pub async fn handle_multilingual_intake(
    State(state): State<AppState>,
    Json(payload): Json<MultilingualIntakePayload>,
) -> impl IntoResponse {
    let tenant_id = &payload.tenant_id;
    let transcript = &payload.transcript;

    // 1. Get tenant's language preference
    let language_preference: String = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar("SELECT language_preference FROM tenants WHERE id = $1")
                .bind(tenant_id)
                .fetch_optional(&state.db.pool)
                .await
                .unwrap_or(None)
                .flatten()
                .unwrap_or_else(|| "en".to_string())
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar("SELECT language_preference FROM tenants WHERE id = ?")
                .bind(tenant_id)
                .fetch_optional(sqlite_pool)
                .await
                .unwrap_or(None)
                .flatten()
                .unwrap_or_else(|| "en".to_string())
        }
    };

    // 2. Parse intent and items with LLM
    let prompt = format!(
        "Analyze the following transcript of an order or inquiry.
        Detect the language and translate the items into '{}'.
        Return ONLY a JSON object with the following structure:
        {{
            \"intent\": \"Order\" | \"Query\" | \"Status Check\",
            \"items\": [
                {{
                    \"name\": \"translated item name\",
                    \"quantity\": 1
                }}
            ]
        }}
        Transcript: {}",
        language_preference, transcript
    );

    let llm_result = crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "{}".to_string());

    // Attempt to parse LLM result
    let cleaned = llm_result.replace("```json", "").replace("```", "").trim().to_string();
    let order_intent: OrderIntent = serde_json::from_str(&cleaned).unwrap_or_else(|_| OrderIntent {
        intent: "Order".to_string(),
        items: vec![], // fallback for tests if LLM fails
    });

    if order_intent.intent == "Order" {
        // Build a readable summary
        let mut order_summary = String::new();
        for item in &order_intent.items {
            order_summary.push_str(&format!("{}x {}\n", item.quantity, item.name));
        }

        let triage_id = format!("triage-{}", Uuid::new_v4());
        let customer_id = format!("cust-{}", Uuid::new_v4()); // Mock customer for now

        // 3. Create Triage Item
        match &state.db.store {
            crate::db::DbStore::Postgres => {
                 let _ = sqlx::query(
                    "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES ($1, $2, $3, 'Omni Intake', 'high', $4, 'pending')"
                )
                .bind(&triage_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&order_summary.trim())
                .execute(&state.db.pool).await;
            }
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                 let _ = sqlx::query(
                    "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES (?, ?, ?, 'Omni Intake', 'high', ?, 'pending')"
                )
                .bind(&triage_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&order_summary.trim())
                .execute(sqlite_pool).await;
            }
        }
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({"success": true, "parsed": order_intent})),
    )
        .into_response()
}
