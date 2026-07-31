#[cfg(test)]
mod tests {
    use super::super::models::{Model as Inbox, channel_web_widget, channel_email, message};
    use uuid::Uuid;
    use chrono::Utc;
    use sea_orm::{Database, DatabaseConnection, Schema};
    use sea_orm::ConnectionTrait;

    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        // Setup schema
        let builder = db.get_database_backend();

        let schema = Schema::new(builder);

        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::Entity))).await;
        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::conversation::Entity))).await;
        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::message::Entity))).await;
        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::contact::Entity))).await;
        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::channel_web_widget::Entity))).await;
        let _ = db.execute(builder.build(&schema.create_table_from_entity(super::super::models::channel_email::Entity))).await;

        db
    }

    #[test]
    fn test_models() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let inbox = Inbox {
            id: inbox_id,
            tenant_id,
            name: "Test Inbox".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(inbox.name, "Test Inbox");

        let web_widget = channel_web_widget::Model {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            website_url: "https://example.com".to_string(),
            widget_color: "#ffffff".to_string(),
        };
        assert_eq!(web_widget.tenant_id, tenant_id);

        let email_channel = channel_email::Model {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            email: "test@example.com".to_string(),
        };
        assert_eq!(email_channel.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_ws_app_state() {
        use super::super::ws::AppState;
        let tenant_id = Uuid::new_v4();

        let db = setup_db().await;
        let state = AppState::new(db);
        let tx = state.get_or_create_channel(tenant_id).await;

        // Subscribe so that send does not return error
        let _rx = tx.subscribe();

        let msg = message::Model {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(),
            sender_id: None,
            content: "Hello".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        tx.send(msg.clone()).unwrap();
    }

    #[tokio::test]
    async fn test_api_send_message_missing_convo() {
        use super::super::api::{ChatApiState, send_message, SendMessageRequest};
        use super::super::ws::AppState as WsAppState;
        use axum::extract::{Path, State};
        use axum::Json;
        use std::sync::Arc;

        let db = setup_db().await;
        let ws_state = Arc::new(WsAppState::new(db.clone()));
        let state = Arc::new(ChatApiState { db, ws_state });

        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();

        let req = SendMessageRequest {
            content: "Test".to_string(),
        };

        let res = send_message(
            Path((tenant_id, conversation_id)),
            State(state),
            Json(req),
        ).await;

        assert!(res.0.is_none(), "Should fail because conversation doesn't exist");
    }

    #[tokio::test]
    async fn test_api_send_message_success() {
        use super::super::api::{ChatApiState, send_message, SendMessageRequest};
        use super::super::ws::AppState as WsAppState;
        use axum::extract::{Path, State};
        use axum::Json;
        use std::sync::Arc;
        use sea_orm::ActiveModelTrait;
        use sea_orm::ActiveValue::Set;

        let db = setup_db().await;
        let ws_state = Arc::new(WsAppState::new(db.clone()));
        let state = Arc::new(ChatApiState { db: db.clone(), ws_state });

        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();

        // Need to insert inbox and contact for foreign keys
        let inbox = super::super::models::ActiveModel {
            id: Set(inbox_id),
            tenant_id: Set(tenant_id),
            name: Set("Inbox 1".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = inbox.insert(&db).await;

        let contact = super::super::models::contact::ActiveModel {
            id: Set(contact_id),
            tenant_id: Set(tenant_id),
            name: Set("Bob".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = contact.insert(&db).await;

        let convo = super::super::models::conversation::ActiveModel {
            id: Set(conversation_id),
            tenant_id: Set(tenant_id),
            inbox_id: Set(inbox_id),
            contact_id: Set(contact_id),
            status: Set("open".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = convo.insert(&db).await.unwrap();

        let req = SendMessageRequest {
            content: "Test".to_string(),
        };

        let res = send_message(
            Path((tenant_id, conversation_id)),
            State(state),
            Json(req),
        ).await;

        assert!(res.0.is_some(), "Should succeed since conversation exists");
        let res_msg = res.0.unwrap();
        assert_eq!(res_msg.content, "Test");
        assert_eq!(res_msg.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_api_get_conversations() {
        use super::super::api::{ChatApiState, get_conversations};
        use super::super::ws::AppState as WsAppState;
        use axum::extract::{Path, State};
        use std::sync::Arc;
        use sea_orm::ActiveModelTrait;
        use sea_orm::ActiveValue::Set;

        let db = setup_db().await;
        let ws_state = Arc::new(WsAppState::new(db.clone()));
        let state = Arc::new(ChatApiState { db: db.clone(), ws_state });

        let tenant_id = Uuid::new_v4();
        let other_tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        // Need to insert inbox and contact for foreign keys
        let inbox = super::super::models::ActiveModel {
            id: Set(inbox_id),
            tenant_id: Set(tenant_id),
            name: Set("Inbox 1".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = inbox.insert(&db).await;

        let contact = super::super::models::contact::ActiveModel {
            id: Set(contact_id),
            tenant_id: Set(tenant_id),
            name: Set("Bob".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = contact.insert(&db).await;

        let convo = super::super::models::conversation::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            inbox_id: Set(inbox_id),
            contact_id: Set(contact_id),
            status: Set("open".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = convo.insert(&db).await.unwrap();

        let convo2 = super::super::models::conversation::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(other_tenant_id),
            inbox_id: Set(inbox_id),
            contact_id: Set(contact_id),
            status: Set("open".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = convo2.insert(&db).await.unwrap();

        let res = get_conversations(Path(tenant_id), State(state)).await;
        assert_eq!(res.0.len(), 1, "Should only return conversations for this tenant");
        assert_eq!(res.0[0].tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_api_get_messages() {
        use super::super::api::{ChatApiState, get_messages};
        use super::super::ws::AppState as WsAppState;
        use axum::extract::{Path, State};
        use std::sync::Arc;
        use sea_orm::ActiveModelTrait;
        use sea_orm::ActiveValue::Set;

        let db = setup_db().await;
        let ws_state = Arc::new(WsAppState::new(db.clone()));
        let state = Arc::new(ChatApiState { db: db.clone(), ws_state });

        let tenant_id = Uuid::new_v4();
        let other_tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();

        // Need to insert inbox and contact for foreign keys
        let inbox = super::super::models::ActiveModel {
            id: Set(inbox_id),
            tenant_id: Set(tenant_id),
            name: Set("Inbox 1".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = inbox.insert(&db).await;

        let contact = super::super::models::contact::ActiveModel {
            id: Set(contact_id),
            tenant_id: Set(tenant_id),
            name: Set("Bob".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = contact.insert(&db).await;

        let convo = super::super::models::conversation::ActiveModel {
            id: Set(conversation_id),
            tenant_id: Set(tenant_id),
            inbox_id: Set(inbox_id),
            contact_id: Set(contact_id),
            status: Set("open".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = convo.insert(&db).await.unwrap();

        let msg1 = super::super::models::message::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            conversation_id: Set(conversation_id),
            sender_id: Set(None),
            content: Set("Hello".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = msg1.insert(&db).await.unwrap();

        let msg2 = super::super::models::message::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(other_tenant_id),
            conversation_id: Set(conversation_id),
            sender_id: Set(None),
            content: Set("Other".to_string()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        let _ = msg2.insert(&db).await.unwrap();

        let res = get_messages(Path((tenant_id, conversation_id)), State(state)).await;
        assert_eq!(res.0.len(), 1, "Should only return messages for this tenant and conversation");
        assert_eq!(res.0[0].tenant_id, tenant_id);
    }
}
