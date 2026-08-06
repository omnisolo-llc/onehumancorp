use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use crate::domain::inbox::repository::InboxRepository;

#[tokio::test]
async fn test_inbox_repo_basics() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());

    // Connect to test database, skip if it fails (not running in real CI database)
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(50))
        .connect(&database_url)
        .await
    {
        Ok(p) => p,
        Err(_) => return, // Skip test if database is not reachable
    };

    let repo = InboxRepository::new(pool);

    // 1. Create a tenant
    let tenant = repo.create_tenant("Test Tenant".to_string()).await.expect("Failed to create tenant");
    assert_eq!(tenant.name, "Test Tenant");

    // 2. Get tenant
    let fetched_tenant = repo.get_tenant(&tenant.id).await.expect("Failed to get tenant");
    assert_eq!(fetched_tenant.id, tenant.id);

    // 3. Create inbox
    let inbox = repo.create_inbox(tenant.id.clone(), "Test Inbox".to_string()).await.expect("Failed to create inbox");
    assert_eq!(inbox.name, "Test Inbox");
    assert_eq!(inbox.tenant_id, tenant.id);

    // 4. Create channel
    let creds = serde_json::json!({"token": "secret"});
    let channel = repo.create_channel(tenant.id.clone(), inbox.id.clone(), "WebWidget".to_string(), creds).await.expect("Failed to create channel");
    assert_eq!(channel.provider_type, "WebWidget");

    // 5. Create contact
    let contact = repo.create_contact(tenant.id.clone(), "John Doe".to_string(), "john@example.com".to_string()).await.expect("Failed to create contact");
    assert_eq!(contact.name, "John Doe");

    // 6. Create conversation
    let conversation = repo.create_conversation(tenant.id.clone(), inbox.id.clone(), contact.id.clone(), "open".to_string()).await.expect("Failed to create conversation");
    assert_eq!(conversation.status, "open");

    // 7. Create message
    let message = repo.create_message(tenant.id.clone(), conversation.id.clone(), "Hello World".to_string(), "contact".to_string(), contact.id.clone()).await.expect("Failed to create message");
    assert_eq!(message.content, "Hello World");

    // 8. Fetch elements
    let fetched_inbox = repo.get_inbox(&tenant.id, &inbox.id).await.expect("Failed to fetch inbox");
    assert_eq!(fetched_inbox.id, inbox.id);

    let fetched_channel = repo.get_channel(&tenant.id, &channel.id).await.expect("Failed to fetch channel");
    assert_eq!(fetched_channel.id, channel.id);

    let fetched_contact = repo.get_contact(&tenant.id, &contact.id).await.expect("Failed to fetch contact");
    assert_eq!(fetched_contact.id, contact.id);

    let fetched_conv = repo.get_conversation(&tenant.id, &conversation.id).await.expect("Failed to fetch conversation");
    assert_eq!(fetched_conv.id, conversation.id);

    let fetched_msg = repo.get_message(&tenant.id, &message.id).await.expect("Failed to fetch message");
    assert_eq!(fetched_msg.id, message.id);
}
