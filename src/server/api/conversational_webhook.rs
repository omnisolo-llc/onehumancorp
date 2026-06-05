use axum::{
    extract::{Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use crate::builder::edge::get_edge_cache;
use crate::domain::repository::models::ConversationalCheckoutSession;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ConversationalWebhookPayload {
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
}

#[derive(Debug, Serialize)]
pub struct ConversationalWebhookResponse {
    pub success: bool,
    pub message: String,
    pub checkout_url: Option<String>,
}

#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_product_price(&self, product_id: &str, tenant_id: &str) -> Result<Option<i64>, String>;
    async fn save_session(&self, session: &ConversationalCheckoutSession) -> Result<(), String>;
}

pub struct DbSessionStore {
    pool: sqlx::PgPool,
}

impl DbSessionStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionStore for DbSessionStore {
    async fn get_product_price(&self, product_id: &str, tenant_id: &str) -> Result<Option<i64>, String> {
        let row = sqlx::query("SELECT price_cents FROM products WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            use sqlx::Row;
            Ok(Some(r.try_get::<i32, _>("price_cents").unwrap_or(1000) as i64))
        } else {
            Ok(None)
        }
    }

    async fn save_session(&self, session: &ConversationalCheckoutSession) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO conversational_checkout_sessions (
                id, tenant_id, customer_id, type, amount, status, inventory_lock_id, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            "#
        )
        .bind(&session.id)
        .bind(&session.tenant_id)
        .bind(&session.customer_id)
        .bind(&session.r#type)
        .bind(session.amount)
        .bind(&session.status)
        .bind(&session.inventory_lock_id)
        .bind(session.created_at)
        .bind(session.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub async fn conversational_webhook_handler(
    axum::extract::State(hub): axum::extract::State<std::sync::Arc<crate::hub::Hub>>,
    Json(payload): Json<ConversationalWebhookPayload>,
) -> impl IntoResponse {
    let store = DbSessionStore::new(hub.pool.clone());
    handle_conversational_webhook_internal(&store, payload).await
}

pub async fn handle_conversational_webhook_internal(
    store: &dyn SessionStore,
    payload: ConversationalWebhookPayload,
) -> impl IntoResponse {
    let cache = get_edge_cache();
    let cache_key = format!("ohc:edge:tenant:{}:product:{}", payload.tenant_id, payload.product_id);

    let mut amount_cents = 1000;
    let mut found = false;

    if let Some(cached_val) = cache.get(&cache_key).await {
        if let Ok(json_val) = serde_json::from_str::<Value>(&cached_val) {
            if let Some(price) = json_val.get("price_cents").and_then(|p| p.as_i64()) {
                amount_cents = price;
                found = true;
            }
        }
    }

    if !found {
        if let Ok(Some(price)) = store.get_product_price(&payload.product_id, &payload.tenant_id).await {
            amount_cents = price;
            found = true;
        }
    }

    if !found {
        tracing::warn!("Product {} not found for tenant {}", payload.product_id, payload.tenant_id);
        return (
            StatusCode::NOT_FOUND,
            Json(ConversationalWebhookResponse {
                success: false,
                message: "Product not found or access denied".to_string(),
                checkout_url: None,
            }),
        );
    }

    let session_id = Uuid::new_v4().to_string();
    let inventory_lock_id = format!("ohc:lock:{}:inventory:{}:{}", payload.tenant_id, payload.product_id, session_id);
    let checkout_url = format!("https://checkout.stripe.com/pay/cs_test_{}", session_id.replace("-", ""));

    let session = ConversationalCheckoutSession {
        id: session_id.clone(),
        tenant_id: payload.tenant_id.clone(),
        customer_id: payload.customer_id.clone(),
        r#type: "product".to_string(),
        amount: amount_cents,
        status: "pending".to_string(),
        inventory_lock_id: Some(inventory_lock_id.clone()),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    if let Err(e) = store.save_session(&session).await {
        tracing::error!("Failed to save conversational checkout session: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ConversationalWebhookResponse {
                success: false,
                message: "Internal server error".to_string(),
                checkout_url: None,
            }),
        );
    }

    (
        StatusCode::OK,
        Json(ConversationalWebhookResponse {
            success: true,
            message: "Quote generated successfully".to_string(),
            checkout_url: Some(checkout_url),
        }),
    )
}
