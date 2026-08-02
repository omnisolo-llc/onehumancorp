use sqlx::{PgPool, Error};
use uuid::Uuid;
use crate::domain::chat::models::{ChatInbox, ChatConversation, ChatMessage};

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<ChatInbox>, Error> {
        let inboxes = sqlx::query_as!(
            ChatInbox,
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(inboxes)
    }

    // Other methods... (stubbed for now since we just need the API)
}
