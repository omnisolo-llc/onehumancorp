use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::Value;
use sqlx::Row;
use crate::db::DB;

#[derive(Deserialize, Serialize)]
pub struct SyncPushRequest {
    pub rows: Vec<Value>,
}

#[derive(Serialize)]
pub struct SyncPushResponse {
    pub status: String,
    pub synced_count: i32,
}

#[derive(Deserialize)]
pub struct SyncPullQuery {
    pub after: Option<String>,
}

#[derive(Serialize)]
pub struct SyncPullResponse {
    pub status: String,
    pub rows: Vec<Value>,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/push", post(push_handler))
        .route("/pull", get(pull_handler))
        .with_state(db)
}

async fn push_handler(
    headers: axum::http::HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<SyncPushRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
    let tenant_id = parsed.0;

    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, "missing tenant identity in session").into_response();
    }

    if payload.rows.is_empty() {
        return Json(SyncPushResponse {
            status: "success".to_string(),
            synced_count: 0,
        }).into_response();
    }

    let mut synced_count = 0;
    let pool = &db.pool;

    match pool.begin().await {
        Ok(mut tx) => {
            if let Err(e) = crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set org context: {}", e)).into_response();
            }

            for row in payload.rows {
                let table = row["table"].as_str().unwrap_or("");
                let id = row["id"].as_str().unwrap_or("");
                let updated_at_str = row["updated_at"].as_str().unwrap_or("");
                let version = row["version"].as_i64().unwrap_or(1);

                if table.is_empty() || id.is_empty() {
                    continue;
                }

                let updated_at = if updated_at_str.is_empty() {
                    chrono::Utc::now()
                } else {
                    chrono::DateTime::parse_from_rfc3339(updated_at_str)
                        .map(|d| d.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now())
                };

                let org_id = row["organization_id"].as_str().unwrap_or(tenant_id.as_str());

                if table == "agent_missions" || table == "shared_tasks" || table == "swarm_tasks" {
                    let status = row["status"].as_str().unwrap_or("");
                    let mission_payload = row["payload"].as_str().unwrap_or("");

                    let query = if table == "agent_missions" {
                        format!("INSERT INTO {} (id, status, payload, organization_id, updated_at, version, _sync_status)
                                 VALUES ($1, $2, $3, $4, $5, $6, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 status = excluded.status, payload = excluded.payload, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table)
                    } else {
                        format!("INSERT INTO {} (id, title, status, payload, organization_id, updated_at, version, _sync_status)
                                 VALUES ($1, 'Sync Task', $2, $3, $4, $5, $6, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 status = excluded.status, payload = excluded.payload, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table)
                    };

                    match sqlx::query(&query)
                        .bind(id)
                        .bind(status)
                        .bind(mission_payload)
                        .bind(org_id)
                        .bind(updated_at)
                        .bind(version as i32)
                        .execute(&mut *tx)
                        .await
                    {
                        Ok(res) => {
                            if res.rows_affected() > 0 {
                                synced_count += 1;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to push sync {}: error={}", table, e);
                        }
                    }
                } else if table == "agent_memories" {
                    let mission_payload = row["payload"].as_str().unwrap_or("");
                    let query = format!("INSERT INTO {} (id, task_id, raw_content, organization_id, updated_at, version, _sync_status)
                                 VALUES ($1, '', $2, $3, $4, $5, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 raw_content = excluded.raw_content, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table);

                    match sqlx::query(&query)
                        .bind(id)
                        .bind(mission_payload)
                        .bind(org_id)
                        .bind(updated_at)
                        .bind(version as i32)
                        .execute(&mut *tx)
                        .await
                    {
                        Ok(res) => {
                            if res.rows_affected() > 0 {
                                synced_count += 1;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to push sync agent_memories: error={}", e);
                        }
                    }
                }
            }

            if let Err(e) = tx.commit().await {
                eprintln!("Failed to commit sync transaction: {}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Transaction commit failed").into_response();
            }
        }
        Err(e) => {
            eprintln!("Failed to begin sync transaction: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Transaction start failed").into_response();
        }
    }

    Json(SyncPushResponse {
        status: "success".to_string(),
        synced_count,
    }).into_response()
}

async fn pull_handler(
    headers: axum::http::HeaderMap,
    State(db): State<Arc<DB>>,
    Query(params): Query<SyncPullQuery>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
    let tenant_id = parsed.0;

    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, "missing tenant identity in session").into_response();
    }

    let mut payload_items = Vec::new();

    let pool = &db.pool;

    let after_time = if let Some(after) = params.after {
        if after.is_empty() {
            chrono::DateTime::<chrono::Utc>::MIN_UTC
        } else {
            chrono::DateTime::parse_from_rfc3339(&after)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::DateTime::<chrono::Utc>::MIN_UTC)
        }
    } else {
        chrono::DateTime::<chrono::Utc>::MIN_UTC
    };

    // Need to use transaction to set org context
    match pool.begin().await {
        Ok(mut tx) => {
            if let Err(e) = crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set org context: {}", e)).into_response();
            }

            let tables = vec!["agent_missions", "shared_tasks", "swarm_tasks", "agent_memories"];
            for table in tables {
                let query = if table == "agent_memories" {
                    format!("SELECT id, '' as status, raw_content as payload, organization_id, updated_at, version FROM {} WHERE updated_at > $1", table)
                } else {
                    format!("SELECT id, status, payload, organization_id, updated_at, version FROM {} WHERE updated_at > $1", table)
                };

                match sqlx::query(&query).bind(after_time).fetch_all(&mut *tx).await {
                    Ok(rows) => {
                        for row in rows {
                            let id: String = row.try_get("id").unwrap_or_default();
                            if id.is_empty() { continue; }
                            let status: String = row.try_get("status").unwrap_or_default();
                            let mission_payload: String = row.try_get("payload").unwrap_or_default();
                            let org_id: String = row.try_get("organization_id").unwrap_or_else(|_| tenant_id.clone());
                            let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now());
                            let version: i32 = row.try_get("version").unwrap_or(1);

                            payload_items.push(serde_json::json!({
                                "table": table,
                                "id": id,
                                "status": status,
                                "payload": mission_payload,
                                "organization_id": org_id,
                                "updated_at": updated_at.to_rfc3339(),
                                "version": version
                            }));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to pull sync {}: {}", table, e);
                    }
                }
            }

            let _ = tx.commit().await;
        }
        Err(e) => {
            eprintln!("Failed to begin pull transaction: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Fetch failed").into_response();
        }
    }

    Json(SyncPullResponse {
        status: "success".to_string(),
        rows: payload_items,
    }).into_response()
}
