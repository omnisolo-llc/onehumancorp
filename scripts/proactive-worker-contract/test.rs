use crate::{
    api::agent_feed::{
        AGENT_FEED_CACHE, AnyAgentFeedListResponse, MobileAgentFeedListResponse,
        get_agent_feed_cache,
    },
    cache::HybridCache,
    db::{DB, DbStore},
    workers::proactive_operations_worker::ProactiveOperationsWorker,
};
use std::{sync::Arc, time::Duration};
#[tokio::test]
async fn actual_spawned_worker_records_inventory_and_invalidates_real_local_cache() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY); INSERT INTO tenants VALUES('owner-a');CREATE TABLE raw_materials(id TEXT PRIMARY KEY,tenant_id TEXT,name TEXT,current_quantity INTEGER,reorder_threshold INTEGER,created_at TEXT,updated_at TEXT);CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload TEXT,proposed_action TEXT,lifecycle_state TEXT,created_at TEXT DEFAULT CURRENT_TIMESTAMP,updated_at TEXT DEFAULT CURRENT_TIMESTAMP);INSERT INTO raw_materials VALUES('recorded-stock','owner-a','Recorded flour',1,5,'2026-01-01','2026-01-01');").execute(&pool).await.unwrap();
    let _ = AGENT_FEED_CACHE.set(Arc::new(HybridCache::new(None)));
    let cache = get_agent_feed_cache();
    cache
        .set_with_tags(
            "owner-a-before",
            AnyAgentFeedListResponse::Mobile(MobileAgentFeedListResponse { items: vec![] }),
            vec!["agent_feed_tenant:owner-a".into()],
            Duration::from_secs(60),
        )
        .await;
    assert!(cache.get("owner-a-before").await.is_some());
    let worker = ProactiveOperationsWorker::new(Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused@127.0.0.1:1/unused")
            .unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    }));
    // The actual unchanged start method typechecks tokio::spawn's Send boundary.
    worker.start();
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            if sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM agent_feed_items WHERE tenant_id='owner-a'",
            )
            .fetch_one(&pool)
            .await
            .unwrap()
                == 1
                && cache.get("owner-a-before").await.is_none()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("actual spawned production wrapper must persist and invalidate");
    let payload: String = sqlx::query_scalar("SELECT context_payload FROM agent_feed_items")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&payload).unwrap()["source_id"],
        "recorded-stock"
    );
    pool.close().await;
}
