use axum::{extract::{State, Path, Query}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use serde_json::json;

#[derive(Serialize)]
pub struct IncidentItem {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub resolution_plan: serde_json::Value,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateIncidentReq {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct TenantQuery {
    pub tenant_id: String,
}

pub async fn list_incidents(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
         return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    match sqlx::query("SELECT id, tenant_id, title, description, status, resolution_plan, created_at FROM incidents WHERE tenant_id = $1 AND status != 'resolved' AND status != 'dismissed' ORDER BY created_at DESC")
        .bind(&query.tenant_id)
        .fetch_all(&mut *tx)
        .await
    {
        Ok(rows) => {
            let items: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                json!({
                    "id": row.get::<String, _>("id"),
                    "tenant_id": row.get::<String, _>("tenant_id"),
                    "title": row.get::<String, _>("title"),
                    "description": row.try_get::<String, _>("description").unwrap_or_default(),
                    "status": row.get::<String, _>("status"),
                    "resolution_plan": row.try_get::<serde_json::Value, _>("resolution_plan").unwrap_or(json!({})),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                })
            }).collect();
            (axum::http::StatusCode::OK, Json(items)).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn create_incident(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<TenantQuery>,
    Json(payload): Json<CreateIncidentReq>,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
         return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    let id = format!("inc-{}", uuid::Uuid::new_v4());

    // Simulate IncidentResolverAgent resolution plan generation
    let resolution_plan = json!({
        "summary": format!("Operator reported '{}'. Automated plan prepared.", payload.title),
        "actions": [
            {
                "type": "notify_staff",
                "description": "Send SMS to repair technician",
                "payload": format!("Urgent repair needed: {}", payload.title)
            },
            {
                "type": "refund_orders",
                "description": "Refund pending affected orders and send apology email",
                "payload": "We are so sorry for the delay. Your order has been refunded."
            },
            {
                "type": "update_inventory",
                "description": "Mark affected items out of stock on menu",
                "payload": null
            }
        ]
    });

    match sqlx::query("INSERT INTO incidents (id, tenant_id, title, description, status, resolution_plan) VALUES ($1, $2, $3, $4, 'proposed', $5) RETURNING id")
        .bind(&id)
        .bind(&query.tenant_id)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(&resolution_plan)
        .fetch_one(&mut *tx)
        .await
    {
        Ok(_) => {
            let _ = tx.commit().await;
            (axum::http::StatusCode::CREATED, Json(json!({"id": id}))).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn approve_incident(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<TenantQuery>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
         return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response();
    }

    // Execute actions (simulated by updating status)
    match sqlx::query("UPDATE incidents SET status = 'resolved', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
        .bind(&id)
        .bind(&query.tenant_id)
        .execute(&mut *tx)
        .await
    {
        Ok(res) => {
            if res.rows_affected() > 0 {
                let _ = tx.commit().await;
                (axum::http::StatusCode::OK, Json(json!({"status": "approved_and_executed"}))).into_response()
            } else {
                (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "incident not found"}))).into_response()
            }
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_incident_item_serialization() {
        let item = IncidentItem {
            id: "inc-1".to_string(),
            tenant_id: "tenant-1".to_string(),
            title: "Test".to_string(),
            description: "Desc".to_string(),
            status: "pending".to_string(),
            resolution_plan: json!({"summary": "test"}),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let serialized = serde_json::to_string(&item).unwrap();
        assert!(serialized.contains("inc-1"));
    }
}
