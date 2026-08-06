#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::db::DB;

    async fn setup_test_db() -> Arc<DB> {
        let pool = crate::db::create_sqlite_pool_for_test().await;

        // Create chat tables in test SQLite DB
        let create_tables = [
            "CREATE TABLE chat_inboxes (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
            "CREATE TABLE chat_contacts (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT,
                email TEXT,
                phone TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
            "CREATE TABLE chat_conversations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT NOT NULL,
                contact_id TEXT NOT NULL,
                assignee_id TEXT,
                status TEXT NOT NULL DEFAULT 'open',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
            "CREATE TABLE chat_messages (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                sender_type TEXT NOT NULL,
                sender_id TEXT,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ];

        for query in create_tables {
            sqlx::query(query).execute(&pool).await.expect("Failed to create test table");
        }

        Arc::new(DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Sqlite(pool),
        })
    }

    fn create_auth_token(auth_store: &::server_auth::Store, tenant_id: &str) -> String {
        let now = chrono::Utc::now();
        auth_store
            .issue_token(&::server_auth::User {
                id: format!("user-{}", tenant_id),
                username: format!("user-{}", tenant_id),
                email: format!("user-{}@example.test", tenant_id),
                password_hash: String::new(),
                roles: vec!["OWNER".to_string()],
                active: true,
                organization_id: Some(tenant_id.to_string()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            })
            .expect("Failed to issue token")
    }

    #[tokio::test]
    async fn test_unauthenticated_requests_blocked() {
        let db = setup_test_db().await;
        let auth_store = Arc::new(::server_auth::Store::new());
        let app = crate::api::native_chat::router(db, auth_store);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/inboxes")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_native_chat_e2e_flow_and_tenant_isolation() {
        let db = setup_test_db().await;
        let auth_store = Arc::new(::server_auth::Store::new());
        let app = crate::api::native_chat::router(db, auth_store.clone());

        // Generate tokens for tenant A and tenant B
        let tenant_id_a = Uuid::new_v4().to_string();
        let tenant_id_b = Uuid::new_v4().to_string();

        let token_a = create_auth_token(&auth_store, &tenant_id_a);
        let token_b = create_auth_token(&auth_store, &tenant_id_b);

        // 1. Create inbox for Tenant A
        let create_inbox_req = Request::builder()
            .uri("/inboxes")
            .method("POST")
            .header("Authorization", format!("Bearer {}", token_a))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "name": "Bakery Customer Care"
            })).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(create_inbox_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_inbox: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let inbox_id_str = created_inbox["id"].as_str().unwrap().to_string();
        assert_eq!(created_inbox["name"], "Bakery Customer Care");
        assert_eq!(created_inbox["tenant_id"], tenant_id_a);

        // 2. Verify List Inboxes returns Tenant A's inbox for Tenant A
        let list_inboxes_req = Request::builder()
            .uri("/inboxes")
            .method("GET")
            .header("Authorization", format!("Bearer {}", token_a))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(list_inboxes_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let inboxes: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        assert_eq!(inboxes.len(), 1);
        assert_eq!(inboxes[0]["id"], inbox_id_str);

        // 3. Verify Tenant B sees ZERO inboxes
        let list_inboxes_b_req = Request::builder()
            .uri("/inboxes")
            .method("GET")
            .header("Authorization", format!("Bearer {}", token_b))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(list_inboxes_b_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let inboxes_b: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        assert!(inboxes_b.is_empty());

        // 4. Create Contact for Tenant A
        let create_contact_req = Request::builder()
            .uri("/contacts")
            .method("POST")
            .header("Authorization", format!("Bearer {}", token_a))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "name": "Maya Bread Lover",
                "email": "maya@lovers.test",
                "phone": "+12345678"
            })).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(create_contact_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_contact: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let contact_id_str = created_contact["id"].as_str().unwrap().to_string();
        assert_eq!(created_contact["name"], "Maya Bread Lover");

        // 5. Verify Contact tenant isolation: Tenant B sees empty contact list
        let list_contacts_b_req = Request::builder()
            .uri("/contacts")
            .method("GET")
            .header("Authorization", format!("Bearer {}", token_b))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(list_contacts_b_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let contacts_b: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        assert!(contacts_b.is_empty());

        // 6. Tenant B tries to start a conversation with Tenant A's contact / inbox (should be rejected/denied)
        let start_conv_for_b_with_a_resources = Request::builder()
            .uri("/conversations")
            .method("POST")
            .header("Authorization", format!("Bearer {}", token_b))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "inbox_id": inbox_id_str,
                "contact_id": contact_id_str
            })).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(start_conv_for_b_with_a_resources).await.unwrap();
        // Since inbox_id is evaluated under RLS context for Tenant B, it is not found => 400 Bad Request
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // 7. Tenant A successfully starts a conversation
        let start_conv_req = Request::builder()
            .uri("/conversations")
            .method("POST")
            .header("Authorization", format!("Bearer {}", token_a))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "inbox_id": inbox_id_str,
                "contact_id": contact_id_str
            })).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(start_conv_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_conversation: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let conversation_id_str = created_conversation["id"].as_str().unwrap().to_string();

        // 8. Tenant B tries to list conversation or list messages for Tenant A's conversation ID (should be blocked)
        let list_messages_b = Request::builder()
            .uri(format!("/conversations/{}/messages", conversation_id_str))
            .method("GET")
            .header("Authorization", format!("Bearer {}", token_b))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(list_messages_b).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // 9. Send message inside the conversation for Tenant A
        let send_msg_req = Request::builder()
            .uri("/messages")
            .method("POST")
            .header("Authorization", format!("Bearer {}", token_a))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "conversation_id": conversation_id_str,
                "sender_type": "contact",
                "content": "Hi, I need 5 sourdough loaves!"
            })).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(send_msg_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_message: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(created_message["content"], "Hi, I need 5 sourdough loaves!");

        // 10. List messages for Tenant A's conversation
        let list_messages_req = Request::builder()
            .uri(format!("/conversations/{}/messages", conversation_id_str))
            .method("GET")
            .header("Authorization", format!("Bearer {}", token_a))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(list_messages_req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let messages: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["content"], "Hi, I need 5 sourdough loaves!");
    }
}
