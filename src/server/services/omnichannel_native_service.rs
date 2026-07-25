use sqlx::{PgPool, FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_type: String,
    pub content: String,
    pub status: String,
}

pub struct OmnichannelNativeRepository {
    pool: PgPool,
}

impl OmnichannelNativeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_conversations(&self, tenant_id: &str) -> Result<Vec<Conversation>, sqlx::Error> {
        let tenant_uuid = Uuid::parse_str(tenant_id).unwrap_or_default();
        sqlx::query_as::<_, Conversation>(
            "SELECT * FROM omnichannel_native_conversations WHERE tenant_id = $1"
        )
        .bind(tenant_uuid)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn fetch_messages(&self, conversation_id: Uuid, tenant_id: &str) -> Result<Vec<Message>, sqlx::Error> {
        let tenant_uuid = Uuid::parse_str(tenant_id).unwrap_or_default();
        sqlx::query_as::<_, Message>(
            r#"
            SELECT m.* FROM omnichannel_native_messages m
            JOIN omnichannel_native_conversations c ON m.conversation_id = c.id
            WHERE m.conversation_id = $1 AND c.tenant_id = $2
            ORDER BY m.created_at ASC
            "#
        )
        .bind(conversation_id)
        .bind(tenant_uuid)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn send_message(&self, conversation_id: Uuid, sender_type: &str, content: &str, tenant_id: &str) -> Result<Message, sqlx::Error> {
        let tenant_uuid = Uuid::parse_str(tenant_id).unwrap_or_default();

        // Verify conversation belongs to tenant
        let conversation_exists = sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM omnichannel_native_conversations WHERE id = $1 AND tenant_id = $2"
        )
        .bind(conversation_id)
        .bind(tenant_uuid)
        .fetch_one(&self.pool)
        .await?;

        if conversation_exists == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let id = Uuid::new_v4();
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO omnichannel_native_messages (id, conversation_id, sender_type, content, status)
            VALUES ($1, $2, $3, $4, 'sent')
            RETURNING *
            "#
        )
        .bind(id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }
}
