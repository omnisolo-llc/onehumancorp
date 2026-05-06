use std::sync::Arc;
use tokio::sync::mpsc;
use crate::hub::Hub;
use crate::db::{DB, DbStore};
use crate::services::dashboard::service::MyDashboardService;
use crate::ohc::app::dashboard_service_server::DashboardService;
use crate::ohc::app::GetDashboardRequest;

#[tokio::test]
async fn test_get_dashboard_mobile_optimized() {
    let (tx, _rx) = mpsc::channel(100);

    // Setup SQLite DB for tests
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:").await.unwrap();

    sqlx::query("CREATE TABLE products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price INTEGER);")
        .execute(&pool)
        .await
        .unwrap();

    // Insert dummy product
    sqlx::query("INSERT INTO products (id, organization_id, title, type, price) VALUES ('p1', 'test_org', 'Test Product', 'virtual', 1000);")
        .execute(&pool)
        .await
        .unwrap();

    // Use an isolated in-memory SQLite database to mock the required database interactions
    let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Sqlite(pool) });

    let hub = Arc::new(Hub::new(tx, sqlx::postgres::PgPoolOptions::new().max_connections(1).acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap()));

    let service = MyDashboardService::new(db, hub);

    // Test mobile optimized (true)
    let req_mobile = GetDashboardRequest {
        organization_id: "test_org".to_string(),
        mobile_optimized: true,
    };

    let res_mobile = service.get_dashboard(tonic::Request::new(req_mobile)).await.unwrap().into_inner();

    assert!(res_mobile.products.is_empty(), "Products should be skipped/empty for mobile optimized");
    assert!(res_mobile.orders.is_empty(), "Orders should be skipped/empty for mobile optimized");

    // Test mobile optimized (false)
    let req_desktop = GetDashboardRequest {
        organization_id: "test_org".to_string(),
        mobile_optimized: false,
    };

    let res_desktop = service.get_dashboard(tonic::Request::new(req_desktop)).await.unwrap().into_inner();
    assert_eq!(res_desktop.products.len(), 1, "Desktop should fetch the 1 product");
    assert_eq!(res_desktop.products[0].id, "p1");
}
