use std::sync::Arc;
use crate::db::{DB, DbStore};
use uuid::Uuid;

pub async fn create_inbox(db: &Arc<DB>, tenant_id: &str, name: &str) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match &db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW())")
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .execute(&db.pool).await?;
        },
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name, created_at, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .execute(pool).await?;
        }
    }
    Ok(())
}

pub async fn create_channel(db: &Arc<DB>, tenant_id: &str, inbox_id: &str, channel_type: &str, config: &serde_json::Value) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match &db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())")
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(channel_type)
                .bind(serde_json::to_string(config).unwrap())
                .execute(&db.pool).await?;
        },
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config, created_at, updated_at) VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(channel_type)
                .bind(serde_json::to_string(config).unwrap())
                .execute(pool).await?;
        }
    }
    Ok(())
}

pub async fn create_contact(db: &Arc<DB>, tenant_id: &str, name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match &db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO chat_contacts (id, tenant_id, name, email, phone, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())")
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .bind(email)
                .bind(phone)
                .execute(&db.pool).await?;
        },
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO chat_contacts (id, tenant_id, name, email, phone, created_at, updated_at) VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .bind(email)
                .bind(phone)
                .execute(pool).await?;
        }
    }
    Ok(())
}

pub async fn create_conversation(db: &Arc<DB>, tenant_id: &str, inbox_id: &str, contact_id: &str, assignee_id: Option<&str>, status: &str) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match &db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())")
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .bind(assignee_id)
                .bind(status)
                .execute(&db.pool).await?;
        },
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .bind(assignee_id)
                .bind(status)
                .execute(pool).await?;
        }
    }
    Ok(())
}

pub async fn insert_message(db: &Arc<DB>, tenant_id: &str, conversation_id: &str, sender_type: &str, sender_id: Option<&str>, content: &str) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match &db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())")
                .bind(id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(sender_type)
                .bind(sender_id)
                .bind(content)
                .execute(&db.pool).await?;
        },
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(sender_type)
                .bind(sender_id)
                .bind(content)
                .execute(pool).await?;
        }
    }
    Ok(())
}
