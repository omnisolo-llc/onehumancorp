use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct OmniTenant {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniInbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniChannel {
    pub id: String,
    pub provider_type: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniContact {
    pub id: String,
    pub tenant_id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniConversation {
    pub id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct OmniMessage {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub status: String,
    pub tenant_id: String,
}

pub struct OmnichannelChatRepo {
    db: Arc<DB>,
}

impl OmnichannelChatRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_tenant(&self, name: String) -> Result<OmniTenant, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniTenant>(
            "INSERT INTO omnichannel_tenants (id, name) VALUES ($1, $2) RETURNING id, name",
        )
        .bind(&id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_tenant(&self, id: &str) -> Result<OmniTenant, sqlx::Error> {
        let record = sqlx::query_as::<_, OmniTenant>(
            "SELECT id, name FROM omnichannel_tenants WHERE id = $1 AND id = current_setting('app.current_tenant', true)",
        )
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_inbox(&self, tenant_id: String, name: String) -> Result<OmniInbox, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniInbox>(
            "INSERT INTO omnichannel_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name",
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_inbox(&self, id: &str) -> Result<OmniInbox, sqlx::Error> {
        let record = sqlx::query_as::<_, OmniInbox>(
            "SELECT id, tenant_id, name FROM omnichannel_inboxes WHERE id = $1 AND tenant_id = current_setting('app.current_tenant', true)",
        )
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_channel(&self, provider_type: String) -> Result<OmniChannel, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniChannel>(
            "INSERT INTO omnichannel_channels (id, provider_type) VALUES ($1, $2) RETURNING id, provider_type",
        )
        .bind(&id)
        .bind(provider_type)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: String, email: Option<String>, phone: Option<String>) -> Result<OmniContact, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniContact>(
            "INSERT INTO omnichannel_contacts (id, tenant_id, email, phone) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, email, phone",
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, inbox_id: String, contact_id: String, status: String, tenant_id: String) -> Result<OmniConversation, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniConversation>(
            "INSERT INTO omnichannel_conversations (id, inbox_id, contact_id, status, tenant_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, inbox_id, contact_id, status, tenant_id",
        )
        .bind(&id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .bind(tenant_id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_conversation(&self, id: &str) -> Result<OmniConversation, sqlx::Error> {
        let record = sqlx::query_as::<_, OmniConversation>(
            "SELECT id, inbox_id, contact_id, status, tenant_id FROM omnichannel_conversations WHERE id = $1 AND tenant_id = current_setting('app.current_tenant', true)",
        )
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, conversation_id: String, content: String, status: String, tenant_id: String) -> Result<OmniMessage, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let record = sqlx::query_as::<_, OmniMessage>(
            "INSERT INTO omnichannel_messages (id, conversation_id, content, status, tenant_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, conversation_id, content, status, tenant_id",
        )
        .bind(&id)
        .bind(conversation_id)
        .bind(content)
        .bind(status)
        .bind(tenant_id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_message(&self, id: &str) -> Result<OmniMessage, sqlx::Error> {
        let record = sqlx::query_as::<_, OmniMessage>(
            "SELECT id, conversation_id, content, status, tenant_id FROM omnichannel_messages WHERE id = $1 AND tenant_id = current_setting('app.current_tenant', true)",
        )
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_omnichannel_chat_repo() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_tenants (id TEXT PRIMARY KEY, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);"
        ).execute(&db.pool).await;
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_inboxes (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);"
        ).execute(&db.pool).await;

        let repo = OmnichannelChatRepo::new(db.clone());
        let tenant_name = "test_tenant_name".to_string();
        let tenant = repo.create_tenant(tenant_name.clone()).await.unwrap();
        assert_eq!(tenant.name, tenant_name);

        let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant.id).execute(&db.pool).await;

        let inbox_name = "test_inbox_name".to_string();
        let inbox = repo.create_inbox(tenant.id.clone(), inbox_name.clone()).await.unwrap();
        assert_eq!(inbox.name, inbox_name);
        assert_eq!(inbox.tenant_id, tenant.id);

        let retrieved_inbox = repo.get_inbox(&inbox.id).await.unwrap();
        assert_eq!(retrieved_inbox.id, inbox.id);
    }
}
