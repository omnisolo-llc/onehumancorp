use super::models::{Contact, Conversation, Inbox, Message};
use sqlx::PgPool;
use uuid::Uuid;

pub struct OmnichannelRepo;

impl OmnichannelRepo {
    pub async fn create_inbox(
        pool: &PgPool,
        tenant_id: Uuid,
        name: String,
        channel_type: String,
        channel_config: Option<serde_json::Value>,
    ) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let config_json = channel_config.map(sqlx::types::Json);

        let row = sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO omnichannel_inboxes (id, tenant_id, name, channel_type, channel_config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, channel_config, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(config_json as Option<sqlx::types::Json<serde_json::Value>>)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn create_contact(
        pool: &PgPool,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        custom_attributes: Option<serde_json::Value>,
    ) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let attr_json = custom_attributes.map(sqlx::types::Json);

        let row = sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO omnichannel_contacts (id, tenant_id, name, email, phone_number, custom_attributes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, email, phone_number, custom_attributes, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .bind(attr_json as Option<sqlx::types::Json<serde_json::Value>>)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn create_conversation(
        pool: &PgPool,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        status: String,
        assignee_id: Option<Uuid>,
    ) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();

        let row = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO omnichannel_conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, last_activity_at, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .bind(assignee_id)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn create_message(
        pool: &PgPool,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_id: Uuid,
        sender_type: String,
        message_type: String,
        content: String,
        external_source_ids: Option<serde_json::Value>,
    ) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let ext_json = external_source_ids.map(sqlx::types::Json);

        let row = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, sender_id, sender_type, message_type, content, external_source_ids)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, sender_id, sender_type, message_type, content, external_source_ids, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_id)
        .bind(sender_type)
        .bind(message_type)
        .bind(content)
        .bind(ext_json as Option<sqlx::types::Json<serde_json::Value>>)
        .fetch_one(pool)
        .await?;

        // Update conversation last_activity_at
        sqlx::query(
            r#"
            UPDATE omnichannel_conversations SET last_activity_at = NOW() WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .execute(pool)
        .await?;

        Ok(row)
    }

    pub async fn list_conversations(
        pool: &PgPool,
        tenant_id: Uuid,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, assignee_id, last_activity_at, created_at, updated_at
            FROM omnichannel_conversations
            WHERE tenant_id = $1
            ORDER BY last_activity_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_conversation_messages(
        pool: &PgPool,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Message>(
            r#"
            SELECT id, tenant_id, conversation_id, sender_id, sender_type, message_type, content, external_source_ids, created_at, updated_at
            FROM omnichannel_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    // Tests will go here
}
