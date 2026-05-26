use crate::services::yield_management::engine::YieldEngine;
use crate::db::DB;

#[tokio::test]
async fn test_yield_engine_get_price_no_profile() {
    let db = DB::new().await.expect("Failed to init db");
    let engine = YieldEngine::new(db);

    // Should return base price when no profile exists
    let price = engine.get_current_price("tenant_123", "target_123", 1000).await.unwrap();
    assert_eq!(price, 1000);
}

#[tokio::test]
async fn test_yield_engine_calculate_optimal_price() {
    // Note: To test logic accurately we would need mock data,
    // but a basic test to verify it works without panicking is a start.
    let db = DB::new().await.expect("Failed to init db");
    let engine = YieldEngine::new(db.clone());

    let tenant_id = uuid::Uuid::new_v4().to_string();
    let target_id = uuid::Uuid::new_v4().to_string();

    // Setup tenant for FK constraint
    sqlx::query!("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant')", tenant_id)
        .execute(&db.pool).await.ok();

    engine.configure_profile(&tenant_id, &target_id, "service", true, 800, 1500).await.unwrap();
    engine.update_capacity(&tenant_id, &target_id, 1, 10).await.unwrap(); // Low capacity (10%)
    engine.add_demand_signal(&tenant_id, &target_id, "page_views", 1.0).await.unwrap(); // High demand

    let price = engine.get_current_price(&tenant_id, &target_id, 1000).await.unwrap();

    // Low capacity = +20% -> 1200
    // High demand = +10% -> 1320
    // Within limits (800 - 1500), so 1320.
    assert_eq!(price, 1320);
}
