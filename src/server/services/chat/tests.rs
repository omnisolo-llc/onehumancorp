use super::models::*;
use super::service::ChatService;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_session_capsule_crud(pool: PgPool) {
    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();
    let conversation_id = Uuid::new_v4();

    // Set tenant context for RLS
    sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id))
        .execute(&service.pool)
        .await
        .unwrap();

    let capsule = service.get_or_create_session_capsule(tenant_id, conversation_id, None).await.unwrap();
    assert_eq!(capsule.tenant_id, tenant_id);
    assert_eq!(capsule.conversation_id, conversation_id);

    let updated_context = serde_json::json!({"key": "value"});
    let updated_capsule = service.update_session_capsule_context(tenant_id, capsule.id, updated_context.clone()).await.unwrap();
    assert_eq!(updated_capsule.context, updated_context);
}

#[test]
fn test_normalize_webhook() {
    let pool = PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();
    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();
    let payload = serde_json::json!({"message": "hello"});

    let envelope = service.normalize_inbound_webhook(payload.clone(), "instagram", tenant_id);
    assert_eq!(envelope.tenant_id, tenant_id);
    assert_eq!(envelope.source, "instagram");
    assert_eq!(envelope.payload, payload);
}
