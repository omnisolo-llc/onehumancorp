use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, };
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ChatContact, ChatConversation, };

#[derive(Debug, Deserialize)]
pub struct WhatsappWebhookPayload {
    pub object: String,
    pub entry: Vec<WhatsappEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappEntry {
    pub id: String,
    pub changes: Vec<WhatsappChange>,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappChange {
    pub value: WhatsappValue,
    pub field: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappValue {
    pub messages: Option<Vec<WhatsappMessage>>,
    pub contacts: Option<Vec<WhatsappContact>>,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<WhatsappText>,
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappText {
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappContact {
    pub profile: WhatsappProfile,
    pub wa_id: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappProfile {
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/webhooks/whatsapp", post(handle_webhook))
        .with_state(state)
}

async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WhatsappWebhookPayload>,
) -> impl IntoResponse {
    for entry in payload.entry {
        for change in entry.changes {
            if let Some(messages) = change.value.messages {
                for msg in messages {
                    if msg.msg_type == "text" {
                        if let Some(text) = msg.text {
                            let mut contact_name = "Unknown".to_string();
                            if let Some(contacts) = &change.value.contacts {
                                if let Some(contact) = contacts.iter().find(|c| c.wa_id == msg.from) {
                                    contact_name = contact.profile.name.clone();
                                }
                            }

                            let tenant_id = Uuid::new_v4();
                            let inbox_id = Uuid::new_v4(); // In real logic this would be derived

                            let contact_record: Option<ChatContact> = sqlx::query_as(
                                r#"
                                INSERT INTO chat_contacts (id, tenant_id, name, phone)
                                VALUES ($1, $2, $3, $4)
                                ON CONFLICT (id) DO NOTHING
                                RETURNING id, tenant_id, name, email, phone, created_at, updated_at
                                "#
                            )
                            .bind(Uuid::new_v4())
                            .bind(tenant_id)
                            .bind(contact_name)
                            .bind(msg.from)
                            .fetch_optional(&state.db)
                            .await
                            .ok()
                            .flatten();

                            let contact_id = contact_record.map(|c| c.id).unwrap_or_else(Uuid::new_v4);

                            let conversation_record: Option<ChatConversation> = sqlx::query_as(
                                r#"
                                INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
                                VALUES ($1, $2, $3, $4, 'open')
                                ON CONFLICT (id) DO NOTHING
                                RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                                "#
                            )
                            .bind(Uuid::new_v4())
                            .bind(tenant_id)
                            .bind(inbox_id)
                            .bind(contact_id)
                            .fetch_optional(&state.db)
                            .await
                            .ok()
                            .flatten();

                            let conversation_id = conversation_record.map(|c| c.id).unwrap_or_else(Uuid::new_v4);

                            let _ = sqlx::query(
                                r#"
                                INSERT INTO chat_messages (id, tenant_id, conversation_id, content, sender_type)
                                VALUES ($1, $2, $3, $4, 'contact')
                                "#
                            )
                            .bind(Uuid::new_v4())
                            .bind(tenant_id)
                            .bind(conversation_id)
                            .bind(text.body)
                            .execute(&state.db)
                            .await;
                        }
                    }
                }
            }
        }
    }

    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_payload() {
        let json = r#"
        {
          "object": "whatsapp_business_account",
          "entry": [{
            "id": "123",
            "changes": [{
              "value": {
                "messaging_product": "whatsapp",
                "metadata": {
                  "display_phone_number": "1234567890",
                  "phone_number_id": "0987654321"
                },
                "contacts": [{
                  "profile": {
                    "name": "Test User"
                  },
                  "wa_id": "11111111111"
                }],
                "messages": [{
                  "from": "11111111111",
                  "id": "wamid.HBgL",
                  "timestamp": "1669865660",
                  "text": {
                    "body": "Hello!"
                  },
                  "type": "text"
                }]
              },
              "field": "messages"
            }]
          }]
        }
        "#;

        let payload: WhatsappWebhookPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.object, "whatsapp_business_account");
        assert_eq!(payload.entry.len(), 1);
        let msg = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
        assert_eq!(msg.text.as_ref().unwrap().body, "Hello!");
    }
}
