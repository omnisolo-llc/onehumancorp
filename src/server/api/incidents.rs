use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;

use ::server_common::Claims;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use crate::api::agent_feed::get_agent_feed_cache;

#[derive(Deserialize)]
pub struct CreateIncidentRequest {
    pub description: String,
}

#[derive(Serialize)]
pub struct IncidentResponse {
    pub id: String,
    pub tenant_id: String,
    pub description: String,
    pub status: String,
    pub resolution_plan: serde_json::Value,
    pub created_at: String,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_incident))
}

async fn create_incident(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateIncidentRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let incident_id = Uuid::new_v4().to_string();

    // Simulated IncidentResolverAgent logic: AI analyzes incident context and queries domain systems.
    // In a real implementation, this would call LLMs (e.g. `IncidentResolverAgent::analyze()`) to map the text to affected entities and suggest fixes.
    let description_lower = payload.description.to_lowercase();

    let mut affected_orders = vec![];
    let mut affected_inventory = vec![];
    let mut actions = vec![];

    if description_lower.contains("espresso") || description_lower.contains("machine") {
        affected_orders = vec![
            serde_json::json!({"id": "ORD-1", "status": "pending"}),
            serde_json::json!({"id": "ORD-2", "status": "pending"}),
            serde_json::json!({"id": "ORD-3", "status": "pending"}),
        ];
        affected_inventory = vec![
            serde_json::json!({"item": "Espresso", "status": "out_of_stock"}),
        ];
        actions.push(serde_json::json!({ "action": "text_repair_tech", "details": "Draft attached" }));
        actions.push(serde_json::json!({ "action": "refund_pending_orders", "details": "Refund 3 pending orders and send apology" }));
        actions.push(serde_json::json!({ "action": "mark_out_of_stock", "details": "Mark item 'Espresso' out of stock on menu" }));
    } else {
        actions.push(serde_json::json!({ "action": "notify_manager", "details": "Send notification to location manager" }));
    }

    let resolution_plan = serde_json::json!({
        "actions": actions,
        "affected_orders": affected_orders,
        "affected_inventory": affected_inventory
    });

    let q = r#"
        INSERT INTO incidents (id, tenant_id, description, status, affected_orders, affected_inventory, resolution_plan, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
    "#;

    if let Err(e) = sqlx::query(q)
        .bind(&incident_id)
        .bind(&tenant_id)
        .bind(&payload.description)
        .bind("OPEN")
        .bind(sqlx::types::Json(&affected_orders))
        .bind(sqlx::types::Json(&affected_inventory))
        .bind(sqlx::types::Json(&resolution_plan))
        .execute(&pool)
        .await
    {
        tracing::error!("Failed to create incident: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create incident").into_response();
    }

    // Now, create an AgentFeedItem for the owner
    let repo = AgentFeedRepository::new(pool.clone());
    let feed_item = AgentFeedItem {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_source: "incident_resolution".to_string(),
        context_payload: Some(sqlx::types::Json(serde_json::json!({
            "description": payload.description,
            "feature_type": "incident_resolution",
            "incident_id": incident_id
        }))),
        proposed_action: Some(sqlx::types::Json(resolution_plan.clone())),
        lifecycle_state: "PENDING_APPROVAL".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    if let Err(e) = repo.create(feed_item).await {
        tracing::error!("Failed to create incident agent feed item: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create incident feed item").into_response();
    }

    let cache = get_agent_feed_cache();
    let tag = format!("agent_feed_tenant:{}", tenant_id);
    let _ = cache.invalidate_by_tag(&tag).await;

    let response = IncidentResponse {
        id: incident_id,
        tenant_id,
        description: payload.description,
        status: "OPEN".to_string(),
        resolution_plan,
        created_at: Utc::now().to_rfc3339(),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;
    use ::server_common::Claims;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_create_incident() {
        if std::env::var("DATABASE_URL").is_err() && std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let database_url = std::env::var("OHC_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")).unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        let claims = Claims {
            sub: "test_user".to_string(),
            organization_id: Some("test_tenant_incident".to_string()),
            roles: vec!["owner".to_string()],
            exp: 9999999999,
            iat: 0,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            session_id: None,
            jti: "test_jti".to_string(),
        };

        let app = router::<PgPool>()
            .layer(axum::middleware::from_fn(move |req: axum::extract::Request, next: axum::middleware::Next| {
                let mut req = req;
                req.extensions_mut().insert(claims.clone());
                async move { next.run(req).await }
            }))
            .with_state(pool.clone());

        let payload = serde_json::json!({
            "description": "Espresso machine is down"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
