use super::models::{ChatChannel, ChatContact, ChatConversation, ChatInbox, ChatMessage};
use sqlx::PgPool;
use uuid::Uuid;

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
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            SELECT $1, $2, $3, $4, $5
            WHERE EXISTS (SELECT 1 FROM chat_inboxes WHERE id = $3 AND tenant_id = $2)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            SELECT $1, $2, $3, $4, $5, 'open'
            WHERE EXISTS (SELECT 1 FROM chat_inboxes WHERE id = $3 AND tenant_id = $2)
              AND EXISTS (SELECT 1 FROM chat_contacts WHERE id = $4 AND tenant_id = $2)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            SELECT $1, $2, $3, $4, $5, $6
            WHERE EXISTS (SELECT 1 FROM chat_conversations WHERE id = $3 AND tenant_id = $2)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{ChatFixture, test_database_url};
    use super::*;

    #[tokio::test]
    async fn test_create_inbox() {
        let fixture = ChatFixture::new().await;
        let service = ChatService::new(fixture.scoped.clone());
        let tenant = Uuid::new_v4();
        let inbox = service
            .create_inbox(tenant, "Support".into())
            .await
            .unwrap();
        let channel = service
            .create_channel(
                tenant,
                inbox.id,
                "widget".into(),
                serde_json::json!({"webhook_url":"https://example.test"}),
            )
            .await
            .unwrap();
        let contact = service
            .create_contact(
                tenant,
                Some("Alice".into()),
                Some("alice@example.test".into()),
                None,
            )
            .await
            .unwrap();
        let conversation = service
            .start_conversation(tenant, inbox.id, contact.id, None)
            .await
            .unwrap();
        let message = service
            .send_message(
                tenant,
                conversation.id,
                "contact".into(),
                Some(contact.id),
                "Hello!".into(),
            )
            .await
            .unwrap();
        let counts = vec![
            fixture.count_as(tenant, "chat_inboxes", inbox.id).await,
            fixture.count_as(tenant, "chat_channels", channel.id).await,
            fixture.count_as(tenant, "chat_contacts", contact.id).await,
            fixture
                .count_as(tenant, "chat_conversations", conversation.id)
                .await,
            fixture.count_as(tenant, "chat_messages", message.id).await,
        ];
        let foreign_tenant = Uuid::new_v4();
        let foreign_counts = vec![
            fixture
                .count_as(foreign_tenant, "chat_inboxes", inbox.id)
                .await,
            fixture
                .count_as(foreign_tenant, "chat_channels", channel.id)
                .await,
            fixture
                .count_as(foreign_tenant, "chat_contacts", contact.id)
                .await,
            fixture
                .count_as(foreign_tenant, "chat_conversations", conversation.id)
                .await,
            fixture
                .count_as(foreign_tenant, "chat_messages", message.id)
                .await,
        ];
        fixture.finish().await;
        assert_eq!(
            foreign_counts,
            vec![0; 5],
            "every stored chat record must remain tenant-isolated"
        );
        assert_eq!(inbox.name, "Support");
        assert_eq!(channel.channel_type, "widget");
        assert_eq!(contact.name.as_deref(), Some("Alice"));
        assert_eq!(conversation.status, "open");
        assert_eq!(message.content, "Hello!");
        assert_eq!(counts, vec![1; 5]);
    }
    #[tokio::test]
    async fn test_rls_isolation() {
        let f = ChatFixture::new().await;
        let service = ChatService::new(f.scoped.clone());
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let inbox = service
            .create_inbox(a, "Tenant1 Inbox".into())
            .await
            .unwrap();
        let owner = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM chat_inboxes WHERE id=$1")
            .bind(inbox.id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
        let counts = [
            f.count_as(a, "chat_inboxes", inbox.id).await,
            f.count_as(b, "chat_inboxes", inbox.id).await,
            f.count_as(a, "chat_inboxes", inbox.id).await,
        ];
        f.finish().await;
        assert_eq!(owner, 1);
        assert_eq!(
            counts,
            [1, 0, 1],
            "RLS must isolate tenants on the same reused connection"
        );
    }
    #[test]
    fn missing_or_unsafe_database_prerequisites_are_visible_errors() {
        assert!(test_database_url(None).is_err());
        for raw in [
            "postgres://127.0.0.1/production",
            "postgres://remote.example/ohc_chat_test",
            "postgres://127.0.0.1/ohc_chat_test?host=remote",
            "postgres://127.0.0.1/ohc_chat_test#x",
        ] {
            assert!(test_database_url(Some(raw)).is_err());
        }
        assert!(test_database_url(Some("postgres://127.0.0.1:55439/ohc_chat_test")).is_ok());
    }
    #[tokio::test]
    async fn foreign_parent_ids_cannot_receive_another_tenants_children() {
        assert_foreign_parent_rejection(false).await;
    }
    #[tokio::test]
    async fn foreign_parent_guards_also_hold_for_the_database_owner() {
        assert_foreign_parent_rejection(true).await;
    }
    async fn assert_foreign_parent_rejection(database_owner: bool) {
        let f = ChatFixture::new().await;
        let service = ChatService::new(if database_owner {
            f.admin.clone()
        } else {
            f.scoped.clone()
        });
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let ai = service.create_inbox(a, "Owned A".into()).await.unwrap();
        let ac = service
            .create_contact(a, Some("A".into()), None, None)
            .await
            .unwrap();
        let bi = service.create_inbox(b, "Private B".into()).await.unwrap();
        let bc = service
            .create_contact(b, Some("B".into()), None, None)
            .await
            .unwrap();
        let conversation = service
            .start_conversation(b, bi.id, bc.id, None)
            .await
            .unwrap();
        let outcomes = [
            service
                .create_channel(a, bi.id, "widget".into(), serde_json::json!({}))
                .await
                .is_err(),
            service
                .start_conversation(a, bi.id, ac.id, None)
                .await
                .is_err(),
            service
                .start_conversation(a, ai.id, bc.id, None)
                .await
                .is_err(),
            service
                .send_message(
                    a,
                    conversation.id,
                    "contact".into(),
                    Some(ac.id),
                    "Unaccepted message".into(),
                )
                .await
                .is_err(),
        ];
        let foreign =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM chat_messages WHERE tenant_id=$1")
                .bind(a)
                .fetch_one(&f.admin)
                .await
                .unwrap();
        f.finish().await;
        assert_eq!(
            outcomes, [true; 4],
            "RLS must not permit cross-tenant parent relationships"
        );
        assert_eq!(foreign, 0);
    }
    #[tokio::test]
    async fn missing_parent_errors_do_not_poison_later_tenant_context() {
        let f = ChatFixture::new().await;
        let service = ChatService::new(f.scoped.clone());
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let missing = service
            .send_message(
                a,
                Uuid::new_v4(),
                "contact".into(),
                None,
                "No parent".into(),
            )
            .await;
        let context: Option<String> =
            sqlx::query_scalar("SELECT NULLIF(current_setting('app.current_tenant_id',true),'')")
                .fetch_one(&f.scoped)
                .await
                .unwrap();
        let inbox = service.create_inbox(b, "Next owner".into()).await.unwrap();
        let counts = [
            f.count_as(b, "chat_inboxes", inbox.id).await,
            f.count_as(a, "chat_inboxes", inbox.id).await,
        ];
        f.finish().await;
        assert!(missing.is_err());
        assert_eq!(context, None);
        assert_eq!(counts, [1, 0]);
    }
    #[tokio::test]
    async fn actual_sql_failures_are_not_successful_empty_results() {
        let f = ChatFixture::new().await;
        let service = ChatService::new(f.scoped.clone());
        let a = Uuid::new_v4();
        sqlx::query("DROP TABLE chat_messages")
            .execute(&f.admin)
            .await
            .unwrap();
        let write = service
            .send_message(
                a,
                Uuid::new_v4(),
                "contact".into(),
                None,
                "No schema".into(),
            )
            .await;
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM chat_messages")
            .fetch_one(&f.scoped)
            .await;
        f.finish().await;
        assert!(write.is_err());
        assert!(count.is_err());
    }
}
