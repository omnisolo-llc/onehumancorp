use super::*;
use sqlx::PgPool;

#[tokio::test]
async fn test_agent_feed_repo() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
    if let Ok(pool) = PgPool::connect(&db_url).await {
        let repo = AgentFeedRepository::new(pool.clone());

        let item = AgentFeedItem {
            id: "test-id-1".to_string(),
            tenant_id: "test-tenant-1".to_string(),
            event_source: "Instagram DM".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        repo.create(&item).await.unwrap();
        let fetched = repo.get("test-tenant-1", "test-id-1").await.unwrap().unwrap();
        assert_eq!(fetched.id, item.id);

        repo.update_state("test-tenant-1", "test-id-1", "APPROVED").await.unwrap();
        let fetched = repo.get("test-tenant-1", "test-id-1").await.unwrap().unwrap();
        assert_eq!(fetched.lifecycle_state, "APPROVED");

        let list = repo.list_pending("test-tenant-1").await.unwrap();
        assert_eq!(list.len(), 0); // because it's approved

        // cleanup
        sqlx::query("DELETE FROM agent_feed_items WHERE id = 'test-id-1'").execute(&pool).await.unwrap();
    }
}
