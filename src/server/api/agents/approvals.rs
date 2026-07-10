use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::utils::cache::HybridCache;
use std::sync::OnceLock;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;

pub static APPROVALS_CACHE: OnceLock<HybridCache<ApprovalsResponse>> = OnceLock::new();


#[derive(Serialize, Deserialize, Clone)]
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<ApprovalRequest>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct DecisionRequest {
    pub approved: bool,
    pub edited_payload: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}


async fn simulate_promoter_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let description = "New product detected! Schedule a post to drive sales?";
    let product_name = "New Collection";
    let parsed = serde_json::json!({
        "tiktok": "Check out our new product!

⚡ Powered by OHC",
        "instagram": "New arrival! Link in bio.

⚡ Powered by OHC",
        "facebook": "We just added a new product to our store.

⚡ Powered by OHC",
        "feature_type": "social_post_draft",
        "product_name": product_name
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Marketing,
        format!("Draft Social Post: {}", product_name),
        tenant_id.clone(),
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        parsed.clone(),
    ).await {
        Ok(_) => {
            let pool = crate::db::get_pool();
            let agent_feed_item_id = uuid::Uuid::new_v4().to_string();

            // Insert fallback feed item so it shows up in UI Unified Agent Feed correctly
            let insert_res = if std::env::var("OHC_DATABASE_URL").unwrap_or_default().starts_with("sqlite") || std::env::var("OHC_DATABASE_URL").is_err() {
                sqlx::query(
                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                )
                .bind(&agent_feed_item_id)
                .bind(&tenant_id)
                .bind("marketing")
                .bind(serde_json::json!({ "description": description, "feature_type": "social_post_draft" }).to_string())
                .bind(parsed.to_string())
                .execute(&pool).await.map(|_| ())
            } else {
                sqlx::query(
                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                )
                .bind(&agent_feed_item_id)
                .bind(&tenant_id)
                .bind("marketing")
                .bind(serde_json::json!({ "description": description, "feature_type": "social_post_draft" }))
                .bind(&parsed)
                .execute(&pool).await.map(|_| ())
            };
            if let Err(e) = insert_res {
                tracing::error!("Failed to insert simulated agent feed item: {}", e);
            }

            (StatusCode::OK, Json(DecisionResponse { success: true })).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to simulate promoter draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/activity", get(list_activity_feed))
        .route("/ledger", get(list_ledger_entries))
        .route("/simulate-smart-pricing", post(simulate_smart_pricing))
        .route("/simulate-quote-draft", post(simulate_quote_draft))
        .route("/simulate-stockout-reorder", post(simulate_stockout_reorder))
        .route("/simulate-ambassador-draft", post(simulate_ambassador_draft))
        .route("/simulate-promoter-draft", post(simulate_promoter_draft))
        .route("/simulate-dispute-resolution", post(simulate_dispute_resolution))
        .route("/simulate-newsletter-draft", post(simulate_newsletter_draft))
        .route("/simulate-autonomous-booking-quote", post(simulate_autonomous_booking_quote))
        .route("/simulate-invoice-draft", post(simulate_invoice_draft))
        .route("/simulate-invoice-followup", post(simulate_invoice_followup))
        .route("/simulate-lead-recovery", post(simulate_lead_recovery))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
}


async fn simulate_stockout_reorder(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.simulate_stockout_restock_and_price(&tenant_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate stockout reorder: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_newsletter_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "newsletter_draft",
        "subject": "Your Weekly Update: 3 New Summer Dresses!",
        "content_preview": "Hey everyone, we just restocked our popular summer dresses. Click here to check them out...",
        "audience": "All Newsletter Subscribers",
        "draft_copy": "Hey everyone, we just restocked our popular summer dresses. Click here to check them out...",
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Marketing,
        "Draft weekly newsletter".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate newsletter draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_quote_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "quote_draft",
        "service": "2-Bedroom Apartment Painting",
        "customer_inquiry": "How much to paint a 2-bedroom apartment?",
        "suggested_price": 1200.0,
        "scope": "Prep, Paint, Cleanup for 2-bedroom apartment.",
        "suggested_time": "Tomorrow at 2 PM",
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Sales,
        "Draft quote for 2-Bedroom Apartment Painting".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate quote draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_smart_pricing(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.simulate_smart_pricing(&tenant_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate smart pricing: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_dispute_resolution(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "dispute_resolution",
        "original_message": "The dress arrived damaged",
        "generated_response": "I'm so sorry your dress was damaged. I've processed a $15 refund and marked the item for return.",
        "refund_amount": 15,
        "operational_action": "Mark 1 unit as damaged in inventory",
        "inbox_message_id": "msg_simulated_dispute_123",
        "source": "instagram_dm",
        "original_content": "The dress arrived damaged",
        "sender_id": "@customer",
        "customer_id": "cust_simulated_dispute_123",
        "past_orders": "Returning Customer (2 past orders).",
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::CustomerSuccess,
        "Draft dispute resolution for review".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate dispute resolution: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_ambassador_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "ambassador_reply",
        "original_message": "Do you have vegan chocolate cake available for Saturday?",
        "generated_response": "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?",
        "context_used": "Found 3 vegan chocolate cakes in inventory for Saturday.",
        "inbox_message_id": "msg_simulated_123",
        "source": "instagram_dm",
        "original_content": "Do you have vegan chocolate cake available for Saturday?",
        "sender_id": "@customer",
        "customer_id": "cust_simulated_123",
        "past_orders": "Returning Customer (2 past orders).",
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::CustomerSuccess,
        "Customer Inquiry Reply Draft".to_string(),
        tenant_id.clone(),
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload.clone(),
    ).await {
        Ok(_) => {
            (StatusCode::OK, Json(DecisionResponse { success: true })).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to simulate ambassador draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}


async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!("approvals:{}:{}:{}:{}", tenant_id, query.cursor.as_deref().unwrap_or("none"), limit, mobile_optimized);
    let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, Json(cached)).into_response();
        }

        let tenant_id_bg = tenant_id.clone();
        let cursor_bg = query.cursor.clone();
        let orchestrator_bg = orchestrator.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let mut approvals = orchestrator_bg.get_pending_approvals(&tenant_id_bg, cursor_bg, limit as i64).await;
            if mobile_optimized {
                for a in &mut approvals {
                    a.payload = None;
                }
            }
            let next_cursor = if approvals.len() == limit {
                approvals.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(&cache_key_bg, ApprovalsResponse { pending_approvals: approvals, next_cursor }, std::time::Duration::from_secs(10)).await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let mut approvals = orchestrator.get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64).await;
    if mobile_optimized {
        for a in &mut approvals {
            a.payload = None;
        }
    }

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    };
    cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(10)).await;

    (StatusCode::OK, Json(response)).into_response()
}


async fn list_activity_feed(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!("activity_feed:{}:{}:{}:{}", tenant_id, query.cursor.as_deref().unwrap_or("none"), limit, mobile_optimized);
    let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, Json(cached)).into_response();
        }

        let tenant_id_bg = tenant_id.clone();
        let cursor_bg = query.cursor.clone();
        let orchestrator_bg = orchestrator.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let mut activities = orchestrator_bg.get_activity_feed(&tenant_id_bg, cursor_bg, limit as i64).await;
            if mobile_optimized {
                for a in &mut activities {
                    a.payload = None;
                }
            }
            let next_cursor = if activities.len() == limit {
                activities.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(&cache_key_bg, ApprovalsResponse { pending_approvals: activities, next_cursor }, std::time::Duration::from_secs(10)).await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let mut activities = orchestrator.get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64).await;
    if mobile_optimized {
        for a in &mut activities {
            a.payload = None;
        }
    }

    let next_cursor = if activities.len() == limit {
        activities.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: activities,
        next_cursor,
    };
    cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(10)).await;

    (StatusCode::OK, Json(response)).into_response()
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.decide_approval(&id, &tenant_id, payload.approved, payload.edited_payload).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
// Support for AI Agent Department Architecture


async fn list_ledger_entries(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "entries": [] }))).into_response(),
    };

    let limit = query.limit.unwrap_or(50);

    match orchestrator.get_ledger_entries(&tenant_id, limit as i64).await {
        Ok(entries) => (StatusCode::OK, Json(serde_json::json!({ "entries": entries }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_approvals_cache_initialization() {
        let tenant_id = "test_tenant";
        let cache_key = format!("approvals:{}:none:20:false", tenant_id);
        let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(None));

        let initial_val = cache.get(&cache_key).await;
        assert!(initial_val.is_none(), "Cache should be empty initially");

        let dummy_resp = ApprovalsResponse {
            pending_approvals: vec![],
            next_cursor: None,
        };

        cache.set(&cache_key, dummy_resp.clone(), std::time::Duration::from_secs(60)).await;

        let cached_val = cache.get(&cache_key).await;
        assert!(cached_val.is_some(), "Cache should hit after set");
    }
}

async fn simulate_invoice_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "invoice_draft",
        "project_name": "Website Redesign",
        "milestone_name": "Phase 1 Complete",
        "amount_cents": 250000,
        "customer_id": "cust_simulated_invoice_123",
        "inbox_message_id": "msg_simulated_invoice_123"
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Finance,
        "Draft invoice for completed project milestone".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate invoice draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_invoice_followup(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "invoice_followup",
        "invoice_id": "inv_simulated_12345",
        "original_message": "Invoice inv_simulated_12345 is overdue.",
        "generated_response": "Hi there, just checking in to see if you received invoice inv_simulated_12345. Let us know if you have any questions!",
        "operational_action": "Draft personalized reminder",
        "customer_id": "cust_simulated_12345",
        "suggested_channel": "email"
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Finance,
        "Draft personalized invoice follow-up for review".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate invoice followup: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_autonomous_booking_quote(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-test-tenant-id").and_then(|h| h.to_str().ok()).unwrap_or("test_tenant").to_string();

    let proposed_slot_id = uuid::Uuid::new_v4().to_string();

    // Acquire Redis Redlock for the slot
    if let Ok(redis_url) = std::env::var("OHC_REDIS_URL").or_else(|_| std::env::var("REDIS_URL")) {
        if let Ok(redis_lock) = crate::orchestration::queue::redis_lock::RedisLock::new(&redis_url) {
            let _ = redis_lock.acquire_lock(&tenant_id, "booking_slot", &proposed_slot_id, 600).await;
        }
    }

    let payload = serde_json::json!({
        "feature_type": "autonomous_quote",
        "service": "Emergency Handyman Service",
        "customer_inquiry": "My sink is leaking, can you come today?",
        "suggested_price": 180.00,
        "scope": "Emergency leak repair including standard parts.",
        "proposed_slots": [
            { "start_time": "2024-10-15T14:00:00Z", "end_time": "2024-10-15T15:00:00Z" },
            { "start_time": "2024-10-15T16:00:00Z", "end_time": "2024-10-15T17:00:00Z" }
        ],
        "proposed_slot_id": proposed_slot_id,
        "require_deposit": true,
        "deposit_amount_cents": 9000,
        "inbox_message_id": "msg_simulated_quote_123"
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Sales,
        "Draft quote and propose schedule for Emergency Handyman Service".to_string(),
        tenant_id.clone(),
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate autonomous booking quote: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_lead_recovery(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "lead_recovery",
        "description": "A potential customer hasn't received a follow-up in over 2 hours.",
        "draft_reply": "Hi there! This is Carlos's assistant. He's on a job right now, but how can we help? We can usually schedule a visit for tomorrow.",
        "inbox_message_id": "msg-lead-recovery"
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::CustomerSuccess,
        "Follow up on missed lead".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate lead recovery: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}
