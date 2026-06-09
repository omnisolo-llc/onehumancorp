use sqlx::PgPool;
use uuid::Uuid;
use crate::services::agent_feed::service::{AgentFeedService, AgentType, CardType, CardStatus};

#[tokio::test]
async fn test_create_and_list_cards() {
    let pool_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    if let Ok(pool) = PgPool::connect(&pool_url).await {
        let service = AgentFeedService::new(pool.clone());
        let tenant_id = Uuid::new_v4();

        let card = service.create_card(
            tenant_id,
            AgentType::Ambassador,
            CardType::Actionable,
            "Test Card".to_string(),
            "Test Description".to_string(),
            None
        ).await.unwrap();

        assert_eq!(card.title, "Test Card");

        let cards = service.list_pending_cards(tenant_id).await.unwrap();
        assert_eq!(cards.len(), 1);

        let resolved = service.resolve_card(tenant_id, card.id, CardStatus::Approved).await.unwrap();
        assert_eq!(resolved.status, CardStatus::Approved);

        let pending_after = service.list_pending_cards(tenant_id).await.unwrap();
        assert_eq!(pending_after.len(), 0);
    }
}
