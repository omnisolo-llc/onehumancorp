use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage, ChatSlaPolicy, ChatAutomationRule, ChatCannedResponse};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let conversation: ChatConversation = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await?;

        self.evaluate_automation_rules(tenant_id, "conversation_created", &serde_json::to_value(&conversation).unwrap()).await;

        Ok(conversation)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let message: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        self.evaluate_automation_rules(tenant_id, "message_created", &serde_json::to_value(&message).unwrap()).await;

        Ok(message)
    }

    pub async fn create_sla_policy(
        &self,
        tenant_id: Uuid,
        name: String,
        first_response_time_seconds: i32,
        resolution_time_seconds: i32,
    ) -> Result<ChatSlaPolicy, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_sla_policies (id, tenant_id, name, first_response_time_seconds, resolution_time_seconds)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, first_response_time_seconds, resolution_time_seconds, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(first_response_time_seconds)
        .bind(resolution_time_seconds)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_automation_rule(
        &self,
        tenant_id: Uuid,
        name: String,
        trigger_event: String,
        conditions: serde_json::Value,
        actions: serde_json::Value,
    ) -> Result<ChatAutomationRule, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_automation_rules (id, tenant_id, name, trigger_event, conditions, actions, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, true)
            RETURNING id, tenant_id, name, trigger_event, conditions, actions, is_active, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(trigger_event)
        .bind(conditions)
        .bind(actions)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_canned_response(
        &self,
        tenant_id: Uuid,
        short_code: String,
        content: String,
    ) -> Result<ChatCannedResponse, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_canned_responses (id, tenant_id, short_code, content)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, short_code, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(short_code)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn evaluate_automation_rules(&self, tenant_id: Uuid, event_name: &str, payload: &serde_json::Value) {
        let rules: Vec<ChatAutomationRule> = sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, trigger_event, conditions, actions, is_active, created_at, updated_at
            FROM chat_automation_rules
            WHERE tenant_id = $1 AND trigger_event = $2 AND is_active = true
            "#
        )
        .bind(tenant_id)
        .bind(event_name)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        for rule in rules {
            if self.check_conditions(&rule.conditions, payload) {
                self.execute_actions(tenant_id, &rule.actions, payload).await;
            }
        }
    }

    pub fn check_conditions(&self, conditions: &serde_json::Value, payload: &serde_json::Value) -> bool {
        // Simple condition evaluator
        if let Some(cond_array) = conditions.as_array() {
            for cond in cond_array {
                if let (Some(field), Some(op), Some(val)) = (
                    cond.get("field").and_then(|v| v.as_str()),
                    cond.get("operator").and_then(|v| v.as_str()),
                    cond.get("value")
                ) {
                    let actual_val = payload.get(field);
                    match op {
                        "equals" => {
                            if actual_val != Some(val) { return false; }
                        },
                        _ => return false,
                    }
                }
            }
        }
        true
    }

    async fn execute_actions(&self, _tenant_id: Uuid, actions: &serde_json::Value, _payload: &serde_json::Value) {
        // Implement action execution logic (e.g. assign conversation, add tag, send auto-reply)
        if let Some(action_array) = actions.as_array() {
            for action in action_array {
                if let Some(action_type) = action.get("type").and_then(|v| v.as_str()) {
                    match action_type {
                        "send_message" => {
                            // In real system, this might enqueue a job
                            println!("Action: send_message");
                        },
                        _ => {
                            println!("Action: {}", action_type);
                        }
                    }
                }
            }
        }
    }
}
