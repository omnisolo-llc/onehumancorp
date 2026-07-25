use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: String,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: String,
    #[serde(rename = "hub.challenge")]
    pub challenge: String,
}

pub async fn verify_webhook(
    Query(query): Query<VerifyQuery>,
    // In a real implementation we would inject the expected token from config
) -> impl IntoResponse {
    let expected_token = "ohc_whatsapp_webhook_secret"; // This should come from config

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge)
    } else {
        (StatusCode::FORBIDDEN, "Forbidden".to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub value: ChangeValue,
    pub field: String,
}

#[derive(Deserialize, Debug)]
pub struct ChangeValue {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub statuses: Option<Vec<Status>>,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub profile: Profile,
    pub wa_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<Text>,
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub audio: Option<Media>,
    pub document: Option<Media>,
    pub location: Option<Location>,
    pub interactive: Option<Interactive>,
    pub button: Option<Button>,
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: String,
    pub mime_type: Option<String>,
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Debug)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Button {
    pub text: String,
    pub payload: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
}

use std::sync::Arc;
use axum::extract::State;
use ::server_lib::api::meta_webhook::MetaWebhookState;
use redis::AsyncCommands;

pub async fn handle_webhook(
    State(state): State<MetaWebhookState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", payload);

    let mut _processed = false;

    // Optional Redis lock to prevent double processing
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client = redis::Client::open(redis_url);

    for entry in payload.entry {
        for change in entry.changes {
            if change.field != "messages" { continue; }
            let value = change.value;
            let display_phone_number = value.metadata.display_phone_number;

            if let Some(messages) = value.messages {
                for message in messages {
                    let sender_id = message.from;
                    let message_id = message.id;

                    if let Ok(client) = &redis_client {
                        if let Ok(mut con) = client.get_multiplexed_tokio_connection().await {
                            let lock_key = format!("whatsapp_webhook_lock:{}", message_id);
                            let set_res: redis::RedisResult<bool> = con.set_nx(&lock_key, "1").await;
                            if let Ok(true) = set_res {
                                let _ = con.expire::<&str, ()>(&lock_key, 300).await; // 5 min expiry
                            } else {
                                tracing::info!("Skipping duplicate WhatsApp message {}", message_id);
                                continue;
                            }
                        }
                    }

                    let text = if let Some(t) = message.text {
                        t.body
                    } else if let Some(img) = message.image {
                        let caption = img.caption.unwrap_or_default();
                        format!("![Image]({}) {}", img.id, caption).trim().to_string()
                    } else if let Some(audio) = message.audio {
                        format!("[Audio]({})", audio.id)
                    } else if let Some(loc) = message.location {
                        format!("Location: {}, {}", loc.latitude, loc.longitude)
                    } else if let Some(btn) = message.button {
                        format!("Button: {}", btn.text)
                    } else if let Some(inter) = message.interactive {
                        if let Some(br) = inter.button_reply {
                            format!("Interactive: {}", br.title)
                        } else if let Some(lr) = inter.list_reply {
                            format!("Interactive List: {}", lr.title)
                        } else {
                            "Interactive message".to_string()
                        }
                    } else {
                        "".to_string()
                    };

                    let clean_phone_number = display_phone_number.replace("+", "").replace("whatsapp:", "");
                    let pool = &state.db.pool;

                    let resolved_tenant_id = match &state.db.store {
                        ::server_lib::db::DbStore::Postgres => {
                            let tid = sqlx::query_scalar::<_, String>(
                                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                            )
                            .bind(&display_phone_number)
                            .bind(&clean_phone_number)
                            .fetch_optional(pool)
                            .await.unwrap_or(None);

                            tid.unwrap_or_else(|| "test_tenant".to_string())
                        },
                        ::server_lib::db::DbStore::Sqlite(sqlite_pool) => {
                            let tid = sqlx::query_scalar::<_, String>(
                                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                            )
                            .bind(&display_phone_number)
                            .bind(&clean_phone_number)
                            .fetch_optional(sqlite_pool)
                            .await.unwrap_or(None);

                            tid.unwrap_or_else(|| "test_tenant".to_string())
                        }
                    };

                    if !text.is_empty() {
                        tracing::info!("Parsed WhatsApp message from {}: {}", sender_id, text);
                        let source = "whatsapp".to_string();
                        ::server_lib::api::meta_webhook::process_omnichannel_message(&state, resolved_tenant_id, source, sender_id.to_string(), text.to_string()).await;
                        _processed = true;
                    }
                }
            }

            if let Some(statuses) = value.statuses {
                for status in statuses {
                    tracing::info!("Received WhatsApp status: {} for {}", status.status, status.id);
                }
            }
        }
    }

    StatusCode::OK
}
