use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::DB;

#[derive(Deserialize)]
pub struct UiTenantQuery {
    pub tenant_id: Option<String>,
}

pub fn ui_tenant_id(q: &UiTenantQuery) -> String {
    q.tenant_id.clone().unwrap_or_else(|| "default_tenant".to_string())
}

#[derive(Deserialize)]
pub struct TriageActionRequest {
    pub triage_item_id: String,
    pub approved: bool,
}

pub async fn get_triage_pending_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "SELECT item_id as id, priority, source_icon as source, customer_name as customer_id, summary as context, source_event_type, source_event_id, source_payload_json, suggested_actions, agent_draft_id, agent_draft_content as action_payload, agent_context_summary, is_resolved, created_at_unix as created_at FROM triage_items WHERE tenant_id = $1::uuid AND is_resolved = false ORDER BY priority DESC, created_at_unix DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let priority_val: i32 = r.get("priority");
                        let priority_str = match priority_val {
                            3 => "High",
                            2 => "Medium",
                            1 => "Low",
                            _ => "Normal"
                        };

                        let suggested_actions: Option<serde_json::Value> = r.try_get("suggested_actions").ok();
                        let mut action_type = "".to_string();
                        if let Some(actions) = suggested_actions {
                            if let Some(actions_array) = actions.as_array() {
                                if !actions_array.is_empty() {
                                    if let Some(action_type_val) = actions_array[0].get("action_type") {
                                        action_type = action_type_val.as_str().unwrap_or("").to_string();
                                    }
                                }
                            }
                        }

                        serde_json::json!({
                            "id": r.get::<uuid::Uuid, _>("id").to_string(),
                            "priority": priority_str,
                            "source": r.get::<Option<String>, _>("source"),
                            "customer_id": r.get::<Option<String>, _>("customer_id"),
                            "context": r.get::<Option<String>, _>("context"),
                            "action_type": action_type,
                            "action_payload": r.get::<Option<String>, _>("action_payload"),
                            "created_at": r.get::<i64, _>("created_at")
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(e) => {
                    println!("DB Error: {:?}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "SELECT item_id as id, priority, source_icon as source, customer_name as customer_id, summary as context, source_event_type, source_event_id, source_payload_json, suggested_actions, agent_draft_id, agent_draft_content as action_payload, agent_context_summary, is_resolved, created_at_unix as created_at FROM triage_items WHERE tenant_id = ? AND is_resolved = false ORDER BY priority DESC, created_at_unix DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let priority_val: i32 = r.get("priority");
                        let priority_str = match priority_val {
                            3 => "High",
                            2 => "Medium",
                            1 => "Low",
                            _ => "Normal"
                        };

                        let suggested_actions_str: Option<String> = r.try_get("suggested_actions").ok();
                        let suggested_actions: Option<serde_json::Value> = suggested_actions_str.and_then(|s| serde_json::from_str(&s).ok());
                        let mut action_type = "".to_string();
                        if let Some(actions) = suggested_actions {
                            if let Some(actions_array) = actions.as_array() {
                                if !actions_array.is_empty() {
                                    if let Some(action_type_val) = actions_array[0].get("action_type") {
                                        action_type = action_type_val.as_str().unwrap_or("").to_string();
                                    }
                                }
                            }
                        }

                        serde_json::json!({
                            "id": r.get::<String, _>("id"),
                            "priority": priority_str,
                            "source": r.get::<Option<String>, _>("source"),
                            "customer_id": r.get::<Option<String>, _>("customer_id"),
                            "context": r.get::<Option<String>, _>("context"),
                            "action_type": action_type,
                            "action_payload": r.get::<Option<String>, _>("action_payload"),
                            "created_at": r.get::<i64, _>("created_at")
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(e) => {
                    println!("DB Error: {:?}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
                }
            }
        }
    }
}

pub async fn post_triage_action_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<TriageActionRequest>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE triage_items SET is_resolved = true WHERE item_id = $1::uuid AND tenant_id = $2::uuid"
            )
            .bind(&payload.triage_item_id)
            .bind(&tenant_id)
            .execute(&db.pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE triage_items SET is_resolved = true WHERE item_id = ? AND tenant_id = ?"
            )
            .bind(&payload.triage_item_id)
            .bind(&tenant_id)
            .execute(pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}
