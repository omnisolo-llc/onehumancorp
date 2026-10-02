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

    pub async fn get_open_conversations(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            SELECT c.id, c.tenant_id, c.inbox_id, c.contact_id, c.assignee_id,
                   c.status, c.created_at, c.updated_at
            FROM chat_conversations c
            JOIN chat_inboxes i ON i.id = c.inbox_id AND i.tenant_id = c.tenant_id
            JOIN chat_contacts p ON p.id = c.contact_id AND p.tenant_id = c.tenant_id
            WHERE c.tenant_id = $1 AND c.inbox_id = $2 AND c.status = 'open'
            ORDER BY c.updated_at DESC, c.id ASC
            "#,
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&mut *tx)
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
        .bind(sender_id.map(|id| id.to_string()))
        .bind(content)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{ChatFixture, fixture_migrator, test_database_url};
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

    #[tokio::test]
    async fn open_conversations_are_tenant_scoped_for_the_restricted_role() {
        assert_open_conversation_scope(false).await;
    }

    #[tokio::test]
    async fn open_conversations_are_tenant_scoped_for_the_database_owner() {
        assert_open_conversation_scope(true).await;
    }

    async fn assert_open_conversation_scope(database_owner: bool) {
        let f = ChatFixture::new().await;
        let service = ChatService::new(if database_owner {
            f.admin.clone()
        } else {
            f.scoped.clone()
        });
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let ai = service.create_inbox(a, "A inbox".into()).await.unwrap();
        let other_ai = service.create_inbox(a, "A other".into()).await.unwrap();
        let bi = service.create_inbox(b, "B inbox".into()).await.unwrap();
        let ac = service.create_contact(a, None, None, None).await.unwrap();
        let bc = service.create_contact(b, None, None, None).await.unwrap();
        let mut tied = Vec::new();
        for _ in 0..2 {
            tied.push(
                service
                    .start_conversation(a, ai.id, ac.id, None)
                    .await
                    .unwrap()
                    .id,
            );
        }
        let older = service
            .start_conversation(a, ai.id, ac.id, None)
            .await
            .unwrap();
        let closed = service
            .start_conversation(a, ai.id, ac.id, None)
            .await
            .unwrap();
        let other = service
            .start_conversation(a, other_ai.id, ac.id, None)
            .await
            .unwrap();
        let foreign = service
            .start_conversation(b, bi.id, bc.id, None)
            .await
            .unwrap();
        sqlx::query("UPDATE chat_conversations SET updated_at='2026-10-02T12:00:00Z'::timestamptz WHERE id=ANY($1)")
            .bind(&tied).execute(&f.admin).await.unwrap();
        sqlx::query("UPDATE chat_conversations SET updated_at='2026-10-01T12:00:00Z'::timestamptz WHERE id=$1")
            .bind(older.id).execute(&f.admin).await.unwrap();
        sqlx::query("UPDATE chat_conversations SET status='closed', updated_at='2026-10-03T12:00:00Z'::timestamptz WHERE id=$1")
            .bind(closed.id).execute(&f.admin).await.unwrap();
        // Historical/imported rows can have tenant-inconsistent parents even
        // though the current write API rejects them. Reads must fail closed too.
        for (tenant, inbox, contact) in [(a, bi.id, ac.id), (b, ai.id, bc.id), (a, ai.id, bc.id)] {
            sqlx::query("INSERT INTO chat_conversations(id,tenant_id,inbox_id,contact_id,status) VALUES($1,$2,$3,$4,'open')")
                .bind(Uuid::new_v4()).bind(tenant).bind(inbox).bind(contact)
                .execute(&f.admin).await.unwrap();
        }
        let actual = service.get_open_conversations(a, ai.id).await.unwrap();
        let foreign_inbox = service.get_open_conversations(a, bi.id).await.unwrap();
        let wrong_tenant = service.get_open_conversations(b, ai.id).await.unwrap();
        let foreign_actual = service.get_open_conversations(b, bi.id).await.unwrap();
        let other_actual = service
            .get_open_conversations(a, other_ai.id)
            .await
            .unwrap();
        let missing = service
            .get_open_conversations(a, Uuid::new_v4())
            .await
            .unwrap();
        let repeated = service.get_open_conversations(a, ai.id).await.unwrap();
        let context: Option<String> =
            sqlx::query_scalar("SELECT NULLIF(current_setting('app.current_tenant_id',true),'')")
                .fetch_one(&f.scoped)
                .await
                .unwrap();
        f.finish().await;
        tied.sort();
        tied.push(older.id);
        assert_eq!(actual.iter().map(|c| c.id).collect::<Vec<_>>(), tied);
        assert!(
            actual
                .iter()
                .all(|c| c.tenant_id == a && c.inbox_id == ai.id && c.status == "open")
        );
        assert!(
            foreign_inbox.is_empty(),
            "another tenant's inbox must not be readable"
        );
        assert!(
            wrong_tenant.is_empty(),
            "conversation tenant alone cannot authorize a foreign inbox"
        );
        assert_eq!(
            foreign_actual.iter().map(|c| c.id).collect::<Vec<_>>(),
            vec![foreign.id]
        );
        assert_eq!(
            other_actual.iter().map(|c| c.id).collect::<Vec<_>>(),
            vec![other.id]
        );
        assert!(missing.is_empty());
        assert_eq!(repeated.iter().map(|c| c.id).collect::<Vec<_>>(), tied);
        assert_eq!(
            context, None,
            "reads must not leak tenant state on the reused connection"
        );
    }

    #[tokio::test]
    async fn open_conversation_sql_failures_are_errors_without_tenant_leakage() {
        let f = ChatFixture::new().await;
        let service = ChatService::new(f.scoped.clone());
        sqlx::query("ALTER TABLE chat_conversations RENAME TO unavailable_chat_conversations")
            .execute(&f.admin)
            .await
            .unwrap();
        let result = service
            .get_open_conversations(Uuid::new_v4(), Uuid::new_v4())
            .await;
        let context: Option<String> =
            sqlx::query_scalar("SELECT NULLIF(current_setting('app.current_tenant_id',true),'')")
                .fetch_one(&f.scoped)
                .await
                .unwrap();
        let next = service
            .create_inbox(Uuid::new_v4(), "After failed read".into())
            .await;
        f.finish().await;
        assert_eq!(
            result
                .unwrap_err()
                .as_database_error()
                .unwrap()
                .code()
                .as_deref(),
            Some("42P01")
        );
        assert_eq!(context, None);
        assert!(next.is_ok(), "a failed read must roll back its transaction");
    }

    #[tokio::test]
    async fn additive_content_type_migration_preserves_old_messages_and_history() {
        assert_content_type_upgrade(false).await;
    }

    #[tokio::test]
    async fn additive_content_type_migration_preserves_an_existing_column_and_values() {
        assert_content_type_upgrade(true).await;
    }

    async fn assert_content_type_upgrade(already_has_column: bool) {
        let f = ChatFixture::legacy().await;
        let service = ChatService::new(f.scoped.clone());
        let tenant = Uuid::new_v4();
        let inbox = service
            .create_inbox(tenant, "Upgrade".into())
            .await
            .unwrap();
        let contact = service
            .create_contact(tenant, None, None, None)
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
                "Preserve old bytes ✉".into(),
            )
            .await
            .unwrap();
        let before: Vec<(i64, Vec<u8>)> = sqlx::query_as(
            "SELECT version,checksum FROM _sqlx_migrations WHERE success ORDER BY version",
        )
        .fetch_all(&f.admin)
        .await
        .unwrap();
        if already_has_column {
            // A database that already received the upstream column must retain
            // both its existing values and the original migration checksums.
            sqlx::raw_sql("ALTER TABLE chat_messages ADD COLUMN content_type VARCHAR(50) NOT NULL DEFAULT 'text'")
                .execute(&f.admin).await.unwrap();
            sqlx::query("UPDATE chat_messages SET content_type='image' WHERE id=$1")
                .bind(message.id)
                .execute(&f.admin)
                .await
                .unwrap();
        }
        let migrations = fixture_migrator(true).await;
        migrations.run(&f.admin).await.unwrap();
        migrations.run(&f.admin).await.unwrap();
        let saved: (String, String, String) =
            sqlx::query_as("SELECT content,sender_id,content_type FROM chat_messages WHERE id=$1")
                .bind(message.id)
                .fetch_one(&f.admin)
                .await
                .unwrap();
        let next = service
            .send_message(
                tenant,
                conversation.id,
                "contact".into(),
                Some(contact.id),
                "After upgrade".into(),
            )
            .await
            .unwrap();
        let next_content_type: String =
            sqlx::query_scalar("SELECT content_type FROM chat_messages WHERE id=$1")
                .bind(next.id)
                .fetch_one(&f.admin)
                .await
                .unwrap();
        let after: Vec<(i64, Vec<u8>)> = sqlx::query_as(
            "SELECT version,checksum FROM _sqlx_migrations WHERE success ORDER BY version",
        )
        .fetch_all(&f.admin)
        .await
        .unwrap();
        let foreign_count = f
            .count_as(Uuid::new_v4(), "chat_messages", message.id)
            .await;
        f.finish().await;
        assert_eq!(
            before
                .iter()
                .map(|(version, _)| *version)
                .collect::<Vec<_>>(),
            vec![233, 1009, 1021]
        );
        assert_eq!(
            &after[..before.len()],
            before.as_slice(),
            "an additive upgrade must not rewrite applied migration history"
        );
        assert_eq!(after.len(), before.len() + 1);
        let added = migrations.iter().find(|m| m.version == 1024).unwrap();
        assert_eq!(after.last().unwrap(), &(1024, added.checksum.to_vec()));
        assert_eq!(
            saved,
            (
                "Preserve old bytes ✉".into(),
                contact.id.to_string(),
                if already_has_column { "image" } else { "text" }.into()
            )
        );
        assert_eq!(next_content_type, "text");
        assert_eq!(foreign_count, 0);
    }
}
