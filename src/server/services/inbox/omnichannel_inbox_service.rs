use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub provider_type: String,
    pub provider_credentials: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub content: String,
    pub source_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct OmnichannelInboxService {
    pool: PgPool,
}

impl OmnichannelInboxService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest_webhook_payload(
        &self,
        tenant_id: &str,
        inbox_id: &str,
        provider_type: &str,
        source_id: &str,
        sender_type: &str,
        content: &str,
        contact_name: Option<&str>,
        contact_email: Option<&str>,
        contact_phone: Option<&str>,
    ) -> Result<String, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let mut contact_id = None;
        if let Some(email) = contact_email {
             let row = sqlx::query_as::<_, (String,)>(
                r#"SELECT id FROM contacts WHERE tenant_id = $1 AND email = $2 LIMIT 1"#
            )
            .bind(tenant_id)
            .bind(email)
            .fetch_optional(&mut *tx)
            .await?;
            if let Some((id,)) = row {
                contact_id = Some(id);
            }
        }

        if contact_id.is_none() {
            if let Some(phone) = contact_phone {
                let row = sqlx::query_as::<_, (String,)>(
                    r#"SELECT id FROM contacts WHERE tenant_id = $1 AND phone = $2 LIMIT 1"#
                )
                .bind(tenant_id)
                .bind(phone)
                .fetch_optional(&mut *tx)
                .await?;
                if let Some((id,)) = row {
                    contact_id = Some(id);
                }
            }
        }

        let cid = match contact_id {
            Some(id) => id,
            None => {
                let new_cid = Uuid::new_v4().to_string();
                sqlx::query(
                    r#"INSERT INTO contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)"#
                )
                .bind(&new_cid)
                .bind(tenant_id)
                .bind(contact_name)
                .bind(contact_email)
                .bind(contact_phone)
                .execute(&mut *tx)
                .await?;
                new_cid
            }
        };

        let mut conversation_id = None;
        let row = sqlx::query_as::<_, (String,)>(
            r#"SELECT id FROM conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open' LIMIT 1"#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(&cid)
        .fetch_optional(&mut *tx)
        .await?;
        if let Some((id,)) = row {
            conversation_id = Some(id);
        }

        let conv_id = match conversation_id {
            Some(id) => id,
            None => {
                let new_conv_id = Uuid::new_v4().to_string();
                sqlx::query(
                    r#"INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')"#
                )
                .bind(&new_conv_id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(&cid)
                .execute(&mut *tx)
                .await?;
                new_conv_id
            }
        };

        let msg_id = Uuid::new_v4().to_string();

        let result = sqlx::query(
            r#"INSERT INTO messages (id, tenant_id, conversation_id, sender_type, content, source_id) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (tenant_id, source_id) DO NOTHING"#
        )
        .bind(&msg_id)
        .bind(tenant_id)
        .bind(&conv_id)
        .bind(sender_type)
        .bind(content)
        .bind(source_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        if result.rows_affected() > 0 {
             info!("Ingested omnichannel message {} into conversation {}", msg_id, conv_id);
             Ok(msg_id)
        } else {
             info!("Duplicate omnichannel message {} ignored", source_id);
             Ok(msg_id)
        }
    }
}
