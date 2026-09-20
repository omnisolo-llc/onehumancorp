use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WalkupPayload {
    pub message: String,
}

#[derive(Serialize)]
pub struct WalkupResponse {
    pub success: bool,
    pub structured_order: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::DB>,
}

fn signed_tenant_id(claims: &::server_common::Claims) -> Option<&str> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant| !tenant.is_empty() && !tenant.eq_ignore_ascii_case("system"))
}

pub async fn handle_walkup(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<WalkupPayload>,
) -> impl IntoResponse {
    let Some(tenant_id) = signed_tenant_id(&claims) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(WalkupResponse {
                success: false,
                structured_order: None,
            }),
        )
            .into_response();
    };
    let message = payload.message.trim();
    if message.is_empty() || message.chars().count() > 4_000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(WalkupResponse {
                success: false,
                structured_order: None,
            }),
        )
            .into_response();
    }

    let target_language: String = {
        let language = match &state.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("SELECT language_preference FROM tenants WHERE id = $1")
                    .bind(tenant_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten()
                    .and_then(|row| {
                        row.try_get::<Option<String>, _>("language_preference")
                            .ok()
                            .flatten()
                    })
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("SELECT language_preference FROM tenants WHERE id = ?")
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .ok()
                    .flatten()
                    .and_then(|row| {
                        row.try_get::<Option<String>, _>("language_preference")
                            .ok()
                            .flatten()
                    })
            }
        };
        language.unwrap_or_else(|| "en".to_string())
    };

    let prompt = format!(
        "You are a Multilingual Order Interceptor for a small business. Detect the language of the following input, translate it to {}, extract the intent (Order, Query, Status Check, etc.), and extract any items and quantities.\nInput: {}\nReturn JSON format exactly like: {{\"intent\": \"Order\", \"translated_text\": \"3x Chicken Tacos\", \"items\": [\"3x Chicken Tacos\"]}}",
        target_language, message
    );

    let raw_response = match std::env::var("OMNISOLO_TRANSLATION_LLM_PROVIDER")
        .or_else(|_| std::env::var("OMNISOLO_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.trim().is_empty() {
                crate::minimax::LocalLLMClient::new()
                    .reason(&crate::pricing::compression::reduce_tokens(&prompt))
                    .await
                    .unwrap_or_default()
            } else {
                crate::minimax::MinimaxClient::new(api_key)
                    .reason(&crate::pricing::compression::reduce_tokens(&prompt))
                    .await
                    .unwrap_or_default()
            }
        }
        _ => crate::minimax::LocalLLMClient::new()
            .reason(&crate::pricing::compression::reduce_tokens(&prompt))
            .await
            .unwrap_or_default(),
    };

    let clean_res = raw_response
        .trim_matches('`')
        .trim_start_matches("json\n")
        .trim_end();
    if let Ok(translated_json) = serde_json::from_str::<serde_json::Value>(clean_res) {
        let intent = translated_json
            .get("intent")
            .and_then(|v| v.as_str())
            .unwrap_or("Query");
        let translated_text = translated_json
            .get("translated_text")
            .and_then(|v| v.as_str())
            .unwrap_or(message);

        if intent == "Order" {
            let item_id = uuid::Uuid::new_v4().to_string();
            let persisted = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = match state.db.pool.begin().await {
                        Ok(tx) => tx,
                        Err(_) => return storage_failure(),
                    };
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_err() {
                        return storage_failure();
                    }
                    if sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Multilingual Interceptor Agent', 'high', $3, 'pending')",
                    )
                    .bind(&item_id)
                    .bind(tenant_id)
                    .bind(translated_text)
                    .execute(&mut *tx)
                    .await.is_err() { return storage_failure(); }
                    tx.commit().await.map(|_| ())
                }
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES (?, ?, 'Multilingual Interceptor Agent', 'high', ?, 'pending')",
                    )
                    .bind(&item_id)
                    .bind(tenant_id)
                    .bind(translated_text)
                    .execute(pool)
                    .await.map(|_| ())
                }
            };
            if persisted.is_err() {
                return storage_failure();
            }

            return (
                StatusCode::OK,
                Json(WalkupResponse {
                    success: true,
                    structured_order: Some(translated_text.to_string()),
                }),
            )
                .into_response();
        }
        // A understood non-order is not represented as a placed order.
        return (
            StatusCode::OK,
            Json(WalkupResponse {
                success: true,
                structured_order: None,
            }),
        )
            .into_response();
    }

    (
        StatusCode::BAD_GATEWAY,
        Json(WalkupResponse {
            success: false,
            structured_order: None,
        }),
    )
        .into_response()
}

fn storage_failure() -> axum::response::Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(WalkupResponse {
            success: false,
            structured_order: None,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::signed_tenant_id;

    #[test]
    fn walkup_payload_rejects_spoofed_tenant_fields() {
        assert!(
            serde_json::from_str::<super::WalkupPayload>(
                r#"{"message":"One order","tenant_id":"other-business"}"#
            )
            .is_err()
        );
    }

    #[tokio::test]
    async fn walkup_rejects_empty_input_before_database_or_model_access() {
        use axum::Json;
        use axum::extract::{Extension, State};
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let db = crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
                .unwrap(),
            store: crate::db::DbStore::Sqlite(pool),
        };
        let response = super::handle_walkup(
            State(super::AppState {
                db: std::sync::Arc::new(db),
            }),
            Extension(claims(Some("tenant-a"))),
            Json(super::WalkupPayload {
                message: "  ".into(),
            }),
        )
        .await;
        use axum::response::IntoResponse;
        assert_eq!(
            response.into_response().status(),
            axum::http::StatusCode::BAD_REQUEST
        );
    }

    fn claims(organization_id: Option<&str>) -> ::server_common::Claims {
        ::server_common::Claims {
            sub: "user-1".into(),
            exp: i64::MAX,
            iat: 0,
            organization_id: organization_id.map(str::to_string),
            username: String::new(),
            email: String::new(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        }
    }

    #[test]
    fn walkup_tenant_comes_only_from_verified_non_system_claims() {
        assert_eq!(
            signed_tenant_id(&claims(Some(" tenant-7 "))),
            Some("tenant-7")
        );
        assert_eq!(signed_tenant_id(&claims(None)), None);
        assert_eq!(signed_tenant_id(&claims(Some("system"))), None);
        assert_eq!(signed_tenant_id(&claims(Some("  "))), None);
    }
}
