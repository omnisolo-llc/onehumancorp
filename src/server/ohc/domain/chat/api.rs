use super::models::{Conversation, Message};
use uuid::Uuid;

pub fn fetch_conversations(_tenant_id: Uuid) -> Vec<Conversation> {
    vec![]
}

pub fn send_message(tenant_id: Uuid, conversation_id: Uuid, content: &str) -> Result<Message, String> {
    Ok(Message {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id,
        sender_type: "agent".to_string(),
        sender_id: None,
        content: content.to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

pub trait ChannelAdapter {
    fn send_message(&self, message: &Message) -> Result<(), String>;
    fn receive_message(&self, payload: &str) -> Result<Message, String>;
}

pub struct DummyAdapter;
impl ChannelAdapter for DummyAdapter {
    fn send_message(&self, _message: &Message) -> Result<(), String> {
        Ok(())
    }
    fn receive_message(&self, _payload: &str) -> Result<Message, String> {
        Ok(Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "customer".to_string(),
            sender_id: None,
            content: "dummy message".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_conversations() {
        let tenant_id = Uuid::new_v4();
        let convs = fetch_conversations(tenant_id);
        assert!(convs.is_empty());
    }

    #[test]
    fn test_send_message() {
        let tenant_id = Uuid::new_v4();
        let conv_id = Uuid::new_v4();
        let msg = send_message(tenant_id, conv_id, "Hello!").unwrap();
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.tenant_id, tenant_id);
        assert_eq!(msg.conversation_id, conv_id);
    }

    #[test]
    fn test_dummy_adapter() {
        let adapter = DummyAdapter;
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "agent".to_string(),
            sender_id: None,
            content: "test".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert!(adapter.send_message(&msg).is_ok());

        let received = adapter.receive_message("payload").unwrap();
        assert_eq!(received.content, "dummy message");
    }
}


use axum::{routing::{get, post}, Json, Router, extract::Path};

pub fn router<S>() -> Router<S> where S: Clone + Send + Sync + 'static {
    Router::new()
        .route("/api/v1/chat/{tenant_id}/conversations", get(get_conversations))
        .route("/api/v1/chat/{tenant_id}/conversations/{conversation_id}/messages", post(post_message))
}

async fn get_conversations(Path(tenant_id): Path<String>) -> Result<Json<Vec<Conversation>>, axum::http::StatusCode> {
    if let Ok(tenant_uuid) = Uuid::parse_str(&tenant_id) {
        Ok(Json(fetch_conversations(tenant_uuid)))
    } else {
        Err(axum::http::StatusCode::BAD_REQUEST)
    }
}

#[derive(serde::Deserialize)]
pub struct SendMessagePayload {
    pub content: String,
}

async fn post_message(Path((tenant_id, conversation_id)): Path<(String, String)>, Json(payload): Json<SendMessagePayload>) -> Result<Json<Message>, axum::http::StatusCode> {
    if let (Ok(tenant_uuid), Ok(conv_uuid)) = (Uuid::parse_str(&tenant_id), Uuid::parse_str(&conversation_id)) {
        match send_message(tenant_uuid, conv_uuid, &payload.content) {
            Ok(msg) => Ok(Json(msg)),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(axum::http::StatusCode::BAD_REQUEST)
    }
}
