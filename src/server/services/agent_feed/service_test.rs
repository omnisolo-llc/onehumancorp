use super::*;
use tonic::Request;
use crate::db::{DB, DbStore};
use std::sync::Arc;

async fn setup_test_db() -> Arc<DB> {
    let database_url = std::env::var("OHC_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc_test".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    Arc::new(DB { pool, store: DbStore::Sqlite(sqlite_pool) })
}

#[tokio::test]
async fn test_agent_feed_service_crud() {
    std::env::set_var("OHC_DATABASE_URL", "postgres://postgres:postgres@localhost:5432/ohc_test");

    let db = setup_test_db().await;
    let service = MyAgentFeedService::new(db.clone());
    let tenant_id = "test-tenant-123";

    let mut tx = db.pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    sqlx::query("DELETE FROM agent_feed_items WHERE tenant_id = $1")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO agent_feed_items (id, tenant_id, trigger_event, context_payload, proposed_action, state) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind("test_event")
    .bind(serde_json::json!({"foo": "bar"}))
    .bind(serde_json::json!({"action": "reply"}))
    .bind("PENDING")
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();

    let req = GetFeedRequest {
        tenant_id: tenant_id.to_string(),
        limit: 10,
        cursor: "".to_string(),
    };
    let res = service.get_feed(Request::new(req)).await.unwrap().into_inner();
    assert_eq!(res.items.len(), 1);
    assert_eq!(res.items[0].trigger_event, "test_event");
    assert_eq!(res.items[0].state, FeedItemState::Pending as i32);

    let update_req = UpdateFeedItemStateRequest {
        tenant_id: tenant_id.to_string(),
        id: id.to_string(),
        new_state: FeedItemState::Executed as i32,
    };
    let update_res = service.update_feed_item_state(Request::new(update_req)).await.unwrap().into_inner();
    let updated_item = update_res.item.unwrap();
    assert_eq!(updated_item.state, FeedItemState::Executed as i32);

    let req2 = GetFeedRequest {
        tenant_id: tenant_id.to_string(),
        limit: 10,
        cursor: "".to_string(),
    };
    let res2 = service.get_feed(Request::new(req2)).await.unwrap().into_inner();
    assert_eq!(res2.items[0].state, FeedItemState::Executed as i32);

    let req3 = GetFeedRequest {
        tenant_id: "other-tenant".to_string(),
        limit: 10,
        cursor: "".to_string(),
    };
    let res3 = service.get_feed(Request::new(req3)).await.unwrap().into_inner();
    assert_eq!(res3.items.len(), 0);
}
