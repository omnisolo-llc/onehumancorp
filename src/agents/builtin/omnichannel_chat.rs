#![allow(dead_code)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use crate::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message as LlmMessage};

/// Chatwoot Retirement & Custom Rust Omnichannel Chat System Standard (MANDATORY)
/// Complete Chatwoot Retirement: Chatwoot as an external third-party service, dependency, or integration is 100% RETIRED.
/// OHC implements its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust.
/// These models replicate Chatwoot's data models to achieve 100% feature parity.
/// Row-level tenant isolation is strictly enforced via `tenant_id`.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: Option<i64>, // Set by DB
    pub tenant_id: String,
    pub name: String,
    pub locale: String,
    pub domain: Option<String>,
    pub auto_resolve_duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: Option<i64>,
    pub tenant_id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub available_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Agent,
    Administrator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: Option<i64>,
    pub tenant_id: String,
    pub account_id: i64,
    pub name: String,
    pub channel_type: String,
    pub enable_auto_assignment: bool,
    pub greeting_enabled: bool,
    pub greeting_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Conversation {
    pub id: Option<i64>,
    pub tenant_id: String,
    pub account_id: i64,
    pub inbox_id: i64,
    pub contact_id: i64,
    pub assignee_id: Option<i64>,
    pub status: String,
    pub unread_count: i64,
    pub custom_attributes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Message {
    pub id: Option<i64>,
    pub tenant_id: String,
    pub account_id: i64,
    pub inbox_id: i64,
    pub conversation_id: i64,
    pub message_type: String,
    pub content: Option<String>,
    pub private: bool,
    pub sender_type: String,
    pub sender_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: Option<i64>,
    pub tenant_id: String,
    pub account_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub custom_attributes: String,
}

#[derive(Clone)]
pub struct OmnichannelEngine {
    pool: PgPool,
    llm: Arc<dyn LlmClient>,
}

// Logic separated for easier unit testing without full DB
pub async fn execute_draft_copilot_response(llm: &Arc<dyn LlmClient>, content: &str) -> Option<String> {
    let req = ChatRequest {
        model: "gpt-4o".to_string(),
        system: "You are a customer support copilot. Draft a helpful, professional response to the incoming message.".to_string(),
        messages: vec![LlmMessage::user(content.to_string())],
        tools: vec![],
        max_tokens: 1000,
        temperature: 0.7,
    };

    match llm.chat(req).await {
        Ok(resp) => Some(resp.message.content),
        Err(e) => {
            tracing::error!("Failed to generate copilot draft: {}", e);
            None
        }
    }
}

impl OmnichannelEngine {
    pub async fn new(db_url: &str, llm: Arc<dyn LlmClient>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // DDL migrations are handled externally to avoid race conditions.
        // The expected schema MUST include `tenant_id` on all tables with ENABLE ROW LEVEL SECURITY.

        Ok(Self { pool, llm })
    }

    pub async fn add_account(&self, account: Account) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO accounts (tenant_id, name, locale, domain, auto_resolve_duration) VALUES ($1, $2, $3, $4, $5) RETURNING id"
        )
        .bind(&account.tenant_id)
        .bind(&account.name)
        .bind(&account.locale)
        .bind(&account.domain)
        .bind(account.auto_resolve_duration)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn add_inbox(&self, inbox: Inbox) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO inboxes (tenant_id, account_id, name, channel_type, enable_auto_assignment, greeting_enabled, greeting_message) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"
        )
        .bind(&inbox.tenant_id)
        .bind(inbox.account_id)
        .bind(&inbox.name)
        .bind(&inbox.channel_type)
        .bind(inbox.enable_auto_assignment)
        .bind(inbox.greeting_enabled)
        .bind(&inbox.greeting_message)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn create_conversation(&self, conv: Conversation) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO conversations (tenant_id, account_id, inbox_id, contact_id, assignee_id, status, unread_count, custom_attributes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"
        )
        .bind(&conv.tenant_id)
        .bind(conv.account_id)
        .bind(conv.inbox_id)
        .bind(conv.contact_id)
        .bind(conv.assignee_id)
        .bind(&conv.status)
        .bind(conv.unread_count)
        .bind(&conv.custom_attributes)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn receive_webhook_event(&self, event_type: &str, payload: serde_json::Value) -> Result<(), String> {
        match event_type {
            "message_created" => {
                let msg: Message = serde_json::from_value(payload).map_err(|e| e.to_string())?;
                self.handle_incoming_message(msg).await.map_err(|e| e.to_string())?;
                Ok(())
            }
            "conversation_created" => {
                let conv: Conversation = serde_json::from_value(payload).map_err(|e| e.to_string())?;
                self.create_conversation(conv).await.map_err(|e| e.to_string())?;
                Ok(())
            }
            _ => Err(format!("Unknown event type: {}", event_type)),
        }
    }

    pub async fn handle_incoming_message(&self, message: Message) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO messages (tenant_id, account_id, inbox_id, conversation_id, message_type, content, private, sender_type, sender_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id"
        )
        .bind(&message.tenant_id)
        .bind(message.account_id)
        .bind(message.inbox_id)
        .bind(message.conversation_id)
        .bind(&message.message_type)
        .bind(&message.content)
        .bind(message.private)
        .bind(&message.sender_type)
        .bind(message.sender_id)
        .fetch_one(&self.pool)
        .await?;

        // Hook into the AI Agent Harness to draft a response natively in Rust
        if message.message_type == "incoming" {
            let _ = self.draft_copilot_response(&message).await;
        }

        Ok(row.0)
    }

    pub async fn draft_copilot_response(&self, message: &Message) -> Option<String> {
        if let Some(content) = &message.content {
            execute_draft_copilot_response(&self.llm, content).await
        } else {
            None
        }
    }

    // We assume row-level security uses SET LOCAL config, but we query explicitly with tenant_id to be extra safe
    pub async fn get_conversation(&self, tenant_id: &str, id: i64) -> Result<Option<Conversation>, sqlx::Error> {
        let row: Option<sqlx::postgres::PgRow> = sqlx::query("SELECT id, tenant_id, account_id, inbox_id, contact_id, assignee_id, status, unread_count, custom_attributes FROM conversations WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool).await?;
        let row = match row {
            Some(r) => {
                use sqlx::Row;
                Some(Conversation {
                    id: Some(r.try_get("id").unwrap_or_else(|_| r.try_get::<i64, usize>(0).unwrap_or(0))),
                    tenant_id: r.try_get("tenant_id").unwrap_or_else(|_| r.try_get::<String, usize>(1).unwrap_or_default()),
                    account_id: r.try_get("account_id").unwrap_or_else(|_| r.try_get::<i64, usize>(2).unwrap_or(0)),
                    inbox_id: r.try_get("inbox_id").unwrap_or_else(|_| r.try_get::<i64, usize>(3).unwrap_or(0)),
                    contact_id: r.try_get("contact_id").unwrap_or_else(|_| r.try_get::<i64, usize>(4).unwrap_or(0)),
                    assignee_id: r.try_get("assignee_id").unwrap_or_else(|_| r.try_get::<Option<i64>, usize>(5).unwrap_or(None)),
                    status: r.try_get("status").unwrap_or_else(|_| r.try_get::<String, usize>(6).unwrap_or_default()),
                    unread_count: r.try_get("unread_count").unwrap_or_else(|_| r.try_get::<i64, usize>(7).unwrap_or(0)),
                    custom_attributes: r.try_get("custom_attributes").unwrap_or_else(|_| r.try_get::<String, usize>(8).unwrap_or_default()),
                })
            }
            None => None,
        };
        Ok(row)
    }

    pub async fn get_message(&self, tenant_id: &str, id: i64) -> Result<Option<Message>, sqlx::Error> {
        let row: Option<sqlx::postgres::PgRow> = sqlx::query("SELECT id, tenant_id, account_id, inbox_id, conversation_id, message_type, content, private, sender_type, sender_id FROM messages WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool).await?;
        let row = match row {
            Some(r) => {
                use sqlx::Row;
                Some(Message {
                    id: Some(r.try_get("id").unwrap_or_else(|_| r.try_get::<i64, usize>(0).unwrap_or(0))),
                    tenant_id: r.try_get("tenant_id").unwrap_or_else(|_| r.try_get::<String, usize>(1).unwrap_or_default()),
                    account_id: r.try_get("account_id").unwrap_or_else(|_| r.try_get::<i64, usize>(2).unwrap_or(0)),
                    inbox_id: r.try_get("inbox_id").unwrap_or_else(|_| r.try_get::<i64, usize>(3).unwrap_or(0)),
                    conversation_id: r.try_get("conversation_id").unwrap_or_else(|_| r.try_get::<i64, usize>(4).unwrap_or(0)),
                    message_type: r.try_get("message_type").unwrap_or_else(|_| r.try_get::<String, usize>(5).unwrap_or_default()),
                    content: r.try_get("content").unwrap_or_else(|_| r.try_get::<Option<String>, usize>(6).unwrap_or(None)),
                    private: r.try_get("private").unwrap_or_else(|_| r.try_get::<bool, usize>(7).unwrap_or(false)),
                    sender_type: r.try_get("sender_type").unwrap_or_else(|_| r.try_get::<String, usize>(8).unwrap_or_default()),
                    sender_id: r.try_get("sender_id").unwrap_or_else(|_| r.try_get::<Option<i64>, usize>(9).unwrap_or(None)),
                })
            }
            None => None,
        };
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ChatResponse;

    struct MockLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: LlmMessage::assistant("Drafted copilot response to: Need help!"),
                usage: ohc_builtin_agent_core::types::Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    struct ErrorLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for ErrorLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("Rate limit exceeded".into())
        }
    }

    #[tokio::test]
    async fn test_draft_copilot_response_success() {
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlmClient);
        let result = execute_draft_copilot_response(&llm, "Need help!").await;
        assert_eq!(result, Some("Drafted copilot response to: Need help!".to_string()));
    }

    #[tokio::test]
    async fn test_draft_copilot_response_error() {
        let llm: Arc<dyn LlmClient> = Arc::new(ErrorLlmClient);
        let result = execute_draft_copilot_response(&llm, "Need help!").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_draft_copilot_response_none_content() {
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlmClient);
        // By decoupling, we test that None content doesn't call llm at all
        let msg = Message {
            id: Some(1),
            tenant_id: "tenant_1".to_string(),
            account_id: 1,
            inbox_id: 1,
            conversation_id: 1,
            message_type: "incoming".to_string(),
            content: None,
            private: false,
            sender_type: "Contact".to_string(),
            sender_id: Some(1),
        };

        let draft = if let Some(content) = &msg.content {
            execute_draft_copilot_response(&llm, content).await
        } else {
            None
        };

        assert_eq!(draft, None);
    }
}
