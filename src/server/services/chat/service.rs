use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage, ChatMacro, ChatCannedResponse, ChatAssignmentPolicy, ChatAutomationRule};

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
        sqlx::query_as(
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
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
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
        .await
    }

    pub async fn create_macro(
        &self,
        tenant_id: Uuid,
        name: String,
        visibility: String,
        actions: serde_json::Value,
        created_by_id: Option<Uuid>,
    ) -> Result<super::models::ChatMacro, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_macros (id, tenant_id, name, visibility, actions, created_by_id, updated_by_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, name, visibility, actions, created_by_id, updated_by_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(visibility)
        .bind(actions)
        .bind(created_by_id)
        .bind(created_by_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_canned_response(
        &self,
        tenant_id: Uuid,
        short_code: String,
        content: String,
    ) -> Result<super::models::ChatCannedResponse, sqlx::Error> {
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

    pub async fn create_assignment_policy(
        &self,
        tenant_id: Uuid,
        name: String,
        assignment_order: String,
        conversation_priority: String,
        enabled: bool,
        fair_distribution_limit: i32,
        fair_distribution_window: i32,
        exclude_older_than_hours: Option<i32>,
    ) -> Result<super::models::ChatAssignmentPolicy, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_assignment_policies (id, tenant_id, name, assignment_order, conversation_priority, enabled, fair_distribution_limit, fair_distribution_window, exclude_older_than_hours)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, name, assignment_order, conversation_priority, enabled, fair_distribution_limit, fair_distribution_window, exclude_older_than_hours, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(assignment_order)
        .bind(conversation_priority)
        .bind(enabled)
        .bind(fair_distribution_limit)
        .bind(fair_distribution_window)
        .bind(exclude_older_than_hours)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_automation_rule(
        &self,
        tenant_id: Uuid,
        name: String,
        description: Option<String>,
        event_name: String,
        conditions: serde_json::Value,
        actions: serde_json::Value,
        active: bool,
        execution_delay: Option<i32>,
    ) -> Result<super::models::ChatAutomationRule, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_automation_rules (id, tenant_id, name, description, event_name, conditions, actions, active, execution_delay)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, name, description, event_name, conditions, actions, active, execution_delay, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(description)
        .bind(event_name)
        .bind(conditions)
        .bind(actions)
        .bind(active)
        .bind(execution_delay)
        .fetch_one(&self.pool)
        .await
    }
}
