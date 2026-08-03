use super::service::ChatService;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::test]
async fn test_chat_service_crud() {
    let database_url = std::env::var("OHC_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

    // We only run this if we have a pool.
    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => p,
        Err(_) => return,
    };
    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();

    // insert tenant
    sqlx::query("INSERT INTO tenants (id, name, ceo_name) VALUES ($1, 'test', 'test') ON CONFLICT DO NOTHING")
        .bind(&tenant_id)
        .execute(&service.pool).await.unwrap();

    // Set current_tenant_id for RLS
    sqlx::query("SELECT set_config('app.current_tenant_id', $1::text, false)")
        .bind(tenant_id.to_string())
        .execute(&service.pool)
        .await
        .unwrap();

    let inbox = service.create_inbox(tenant_id, "Test Inbox".to_string()).await.unwrap();
    assert_eq!(inbox.name, "Test Inbox");

    let contact = service.create_contact(tenant_id, Some("John Doe".to_string()), None, None).await.unwrap();
    assert_eq!(contact.name, Some("John Doe".to_string()));

    let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
    assert_eq!(conversation.status, "open");

    let message = service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();
    assert_eq!(message.content, "Hello!");
}
