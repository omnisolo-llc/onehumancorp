use axum::{
    extract::{Json, State},
    http::{StatusCode, Request},
    response::IntoResponse,
    middleware::Next,
};
use serde_json::Value;
use uuid::Uuid;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use sqlx::PgPool;

pub async fn handle_whatsapp_webhook(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let entries = payload.get("entry").and_then(|v| v.as_array());
    if let Some(entries) = entries {
        for entry in entries {
            let changes = entry.get("changes").and_then(|v| v.as_array());
            if let Some(changes) = changes {
                for change in changes {
                    let value = change.get("value");
                    if let Some(value) = value {
                        let metadata = value.get("metadata");
                        let to_phone = metadata.and_then(|m| m.get("display_phone_number")).and_then(|p| p.as_str()).unwrap_or("");

                        let tenant_id: Option<String> = sqlx::query_scalar(
                            "SELECT tenant_id FROM channel_whatsapp WHERE phone_number = $1 LIMIT 1"
                        )
                        .bind(&to_phone)
                        .fetch_optional(&pool)
                        .await.unwrap_or(None);

                        let tenant_id = match tenant_id {
                            Some(t) => t,
                            None => {
                                tracing::warn!("Could not find tenant for whatsapp webhook receiver {}", to_phone);
                                continue;
                            }
                        };

                        let messages = value.get("messages").and_then(|v| v.as_array());
                        if let Some(messages) = messages {
                            for message in messages {
                                let from = message.get("from").and_then(|v| v.as_str()).unwrap_or("");
                                let _msg_id = message.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                let text = message.get("text")
                                    .and_then(|v| v.get("body"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");

                                let inbox_id = Uuid::new_v4().to_string();

                                let _ = sqlx::query(
                                    "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, '', 'unread', $6, NOW()) ON CONFLICT DO NOTHING"
                                )
                                .bind(&inbox_id)
                                .bind(&tenant_id)
                                .bind("whatsapp_cloud_api")
                                .bind(text)
                                .bind(text)
                                .bind(from)
                                .execute(&pool)
                                .await;

                                let job_id = Uuid::new_v4().to_string();
                                let payload_json = serde_json::json!({
                                    "message_id": inbox_id,
                                    "inbox_message_id": inbox_id,
                                    "source": "whatsapp_cloud_api",
                                    "content": text,
                                    "sender_id": from
                                });
                                let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                                    .bind(&job_id)
                                    .bind(&tenant_id)
                                    .bind(payload_json.to_string())
                                    .execute(&pool)
                                    .await;
                            }
                        }
                    }
                }
            }
        }
    }

    (StatusCode::OK, "EVENT_RECEIVED").into_response()
}

pub async fn whatsapp_signature_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let sig_str = req.headers().get("x-hub-signature-256").map(|v| v.to_str().unwrap_or("").to_string());
    if let Some(sig_str) = sig_str {

        let app_secret = std::env::var("WHATSAPP_APP_SECRET").unwrap_or_default();
        if app_secret.is_empty() {
             let res = next.run(req).await;
             return Ok(res);
        }

        let (parts, body) = req.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(app_secret.as_bytes()).unwrap();
        mac.update(&bytes);
        let result = mac.finalize();
        let expected_sig = format!("sha256={}", hex::encode(result.into_bytes()));

        if sig_str == expected_sig {
             let new_req = Request::from_parts(parts, axum::body::Body::from(bytes));
             let res = next.run(new_req).await;
             return Ok(res);
        } else {
             return Err(StatusCode::UNAUTHORIZED);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
