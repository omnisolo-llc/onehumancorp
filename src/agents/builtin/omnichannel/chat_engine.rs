use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use super::models::{Conversation, Message};

/// Native Rust Omnichannel Chat Engine replicating Chatwoot core functionality
pub struct ChatEngine {
    pub pool: SqlitePool,
}

impl ChatEngine {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // Initialize tables
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_conversations (
                id TEXT PRIMARY KEY,
                account_id TEXT,
                inbox_id TEXT,
                status TEXT,
                assignee_id TEXT,
                created_at INTEGER
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT,
                content TEXT,
                message_type TEXT,
                sender_id TEXT,
                created_at INTEGER
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn create_conversation(&self, conv: Conversation) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO omnichannel_conversations (id, account_id, inbox_id, status, assignee_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&conv.id)
        .bind(&conv.account_id)
        .bind(&conv.inbox_id)
        .bind(&conv.status)
        .bind(&conv.assignee_id)
        .bind(conv.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn add_message(&self, message: Message) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO omnichannel_messages (id, conversation_id, content, message_type, sender_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&message.id)
        .bind(&message.conversation_id)
        .bind(&message.content)
        .bind(&message.message_type)
        .bind(&message.sender_id)
        .bind(message.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_conversation_history(&self, conversation_id: &str) -> Result<Vec<Message>, String> {
        let records = sqlx::query_as::<_, (String, String, String, String, Option<String>, i64)>(
            "SELECT id, conversation_id, content, message_type, sender_id, created_at
             FROM omnichannel_messages
             WHERE conversation_id = ?
             ORDER BY created_at ASC"
        )
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let messages = records.into_iter().map(|row| Message {
            id: row.0,
            conversation_id: row.1,
            content: row.2,
            message_type: row.3,
            sender_id: row.4,
            created_at: row.5,
        }).collect();

        Ok(messages)
    }
}
