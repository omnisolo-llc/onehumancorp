use super::*;
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt; // for `oneshot`

use std::sync::Arc;

fn make_dummy_state() -> ::server_lib::api::meta_webhook::MetaWebhookState {
    use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};

    struct DummyMeshTransport;

    #[async_trait::async_trait]
    impl MeshTransport for DummyMeshTransport {
        async fn publish(&self, _topic: &str, _msg: Message) -> Result<(), String> {
            Ok(())
        }

        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }

        async fn acquire_lock(&self, _resource_id: &str, _agent_id: &str, _ttl_seconds: u64) -> Result<bool, String> {
            Ok(true)
        }

        async fn release_lock(&self, _resource_id: &str, _agent_id: &str) -> Result<(), String> {
            Ok(())
        }

        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
            Ok(())
        }

        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
            Ok(vec![])
        }
    }

    let (tx, _) = tokio::sync::mpsc::channel(100);
    let pool = sqlx::Pool::<sqlx::Postgres>::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();
    let hub = Arc::new(::server_lib::hub::Hub::new(tx, pool.clone()));

    // Create DB directly
    let db = Arc::new(::server_lib::db::DB {
        pool: pool,
        store: ::server_lib::db::DbStore::Postgres,
    });

    let transport: Arc<dyn MeshTransport> = Arc::new(DummyMeshTransport);
    let mesh = Arc::new(::server_lib::orchestration::mesh::CentrifugeNode::new(transport));

    let orchestrator = Arc::new(::server_lib::orchestration::departments::orchestrator::DepartmentOrchestrator::new(
        db.clone(),
        mesh
    ));

    ::server_lib::api::meta_webhook::MetaWebhookState {
        hub,
        db,
        orchestrator,
    }
}

#[tokio::test]
async fn test_verify_webhook_success() {
    let app = whatsapp_routes(make_dummy_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook?hub.mode=subscribe&hub.verify_token=ohc_whatsapp_webhook_secret&hub.challenge=1158201444")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_verify_webhook_failure() {
    let app = whatsapp_routes(make_dummy_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook?hub.mode=subscribe&hub.verify_token=wrong_token&hub.challenge=1158201444")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[test]
fn test_parse_webhook_payload() {
    let payload = r#"{
      "object": "whatsapp_business_account",
      "entry": [{
        "id": "123456",
        "changes": [{
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "123456789",
              "phone_number_id": "987654321"
            },
            "statuses": [{
              "id": "wamid.123",
              "status": "delivered",
              "timestamp": "1234567890",
              "recipient_id": "111111"
            }],
            "messages": [{
              "from": "111111",
              "id": "wamid.456",
              "timestamp": "1234567891",
              "type": "text",
              "text": {
                "body": "Hello"
              }
            }]
          },
          "field": "messages"
        }]
      }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(webhook.entry[0].changes[0].value.statuses.as_ref().unwrap()[0].status, "delivered");
    assert_eq!(webhook.entry[0].changes[0].value.messages.as_ref().unwrap()[0].msg_type, "text");
    assert_eq!(webhook.entry[0].changes[0].value.messages.as_ref().unwrap()[0].text.as_ref().unwrap().body, "Hello");
}
