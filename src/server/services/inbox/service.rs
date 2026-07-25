use sqlx::PgPool;
use uuid::Uuid;
use tracing::info;

pub struct InboxService {
    pool: PgPool,
}

impl InboxService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO omni_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)"#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(id)
    }

    pub async fn resolve_contact(
        &self,
        tenant_id: Uuid,
        channel_type: &str,
        identifier: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as::<_, (Uuid,)>(
            r#"SELECT contact_id FROM omni_contact_identities WHERE tenant_id = $1 AND channel_type = $2 AND identifier = $3 LIMIT 1"#
        )
        .bind(tenant_id)
        .bind(channel_type)
        .bind(identifier)
        .fetch_optional(&mut *tx)
        .await?;

        let contact_id = match row {
            Some((id,)) => id,
            None => {
                let new_contact_id = Uuid::new_v4();
                sqlx::query(
                    r#"INSERT INTO omni_contacts (id, tenant_id) VALUES ($1, $2)"#
                )
                .bind(new_contact_id)
                .bind(tenant_id)
                .execute(&mut *tx)
                .await?;

                let identity_id = Uuid::new_v4();
                sqlx::query(
                    r#"INSERT INTO omni_contact_identities (id, tenant_id, contact_id, channel_type, identifier) VALUES ($1, $2, $3, $4, $5)"#
                )
                .bind(identity_id)
                .bind(tenant_id)
                .bind(new_contact_id)
                .bind(channel_type)
                .bind(identifier)
                .execute(&mut *tx)
                .await?;

                new_contact_id
            }
        };

        tx.commit().await?;
        Ok(contact_id)
    }

    pub async fn ingest_message(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        sender_type: &str,
        content: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as::<_, (Uuid,)>(
            r#"SELECT id FROM omni_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open' LIMIT 1"#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_optional(&mut *tx)
        .await?;

        let conversation_id = match row {
            Some((id,)) => id,
            None => {
                let new_conv_id = Uuid::new_v4();
                sqlx::query(
                    r#"INSERT INTO omni_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')"#
                )
                .bind(new_conv_id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .execute(&mut *tx)
                .await?;
                new_conv_id
            }
        };

        let msg_id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO omni_messages (id, tenant_id, conversation_id, sender_type, content) VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(msg_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(content)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        info!("Ingested message {} into conversation {}", msg_id, conversation_id);
        Ok(msg_id)
    }
}
