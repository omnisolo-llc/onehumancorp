#[cfg(test)]
mod tests {
use ::server_ohc::app::dashboard_service_server::DashboardService;
use ::server_ohc::app::{GetDeliveryRouteRequest, UpdateRouteStopStatusRequest, AuthInfo};
use tonic::Request;
use crate::db::DbStore;
use super::service::MyDashboardService;

#[tokio::test]
async fn test_get_delivery_route() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Init schema
    let schema = r#"
        CREATE TABLE delivery_routes (id TEXT PRIMARY KEY, organization_id TEXT, driver_id TEXT, status TEXT);
        CREATE TABLE route_stops (id TEXT PRIMARY KEY, route_id TEXT, organization_id TEXT, order_id TEXT, address TEXT, status TEXT, eta_ms BIGINT, sort_order INTEGER, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
        CREATE TABLE department_tasks (id TEXT PRIMARY KEY, tenant_id TEXT, department TEXT, event_type TEXT, payload TEXT, status TEXT);
    "#;
    sqlx::query(schema).execute(&pool).await.unwrap();

    let db = std::sync::Arc::new(crate::db::DB {
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });
    let (tx, _rx) = tokio::sync::mpsc::channel(10);
    let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

    let service = super::service::MyDashboardService::new(db.clone(), hub.clone());

    sqlx::query("INSERT INTO delivery_routes (id, organization_id, driver_id, status) VALUES ('route_1', 'org_1', 'driver_1', 'planning')")
        .execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO route_stops (id, route_id, organization_id, order_id, address, status, eta_ms, sort_order) VALUES ('stop_1', 'route_1', 'org_1', 'order_1', '123 Test St', 'pending', 123456789, 1)")
        .execute(&pool).await.unwrap();

    let mut req = Request::new(GetDeliveryRouteRequest { route_id: "route_1".to_string() });
    req.extensions_mut().insert(AuthInfo {
        spiffe_id: "test".to_string(),
        org_id: "org_1".to_string(),
        agent_id: "test".to_string(),
    });

    let res = service.get_delivery_route(req).await.unwrap().into_inner();
    assert_eq!(res.id, "route_1");
    assert_eq!(res.driver_id, "driver_1");
    assert_eq!(res.stops.len(), 1);
    assert_eq!(res.stops[0].address, "123 Test St");

    let mut update_req = Request::new(UpdateRouteStopStatusRequest {
        route_id: "route_1".to_string(),
        stop_id: "stop_1".to_string(),
        status: "out_for_delivery".to_string(),
    });
    update_req.extensions_mut().insert(AuthInfo {
        spiffe_id: "test".to_string(),
        org_id: "org_1".to_string(),
        agent_id: "test".to_string(),
    });

    let update_res = service.update_route_stop_status(update_req).await.unwrap().into_inner();
    assert_eq!(update_res.status, "out_for_delivery");
    assert_eq!(update_res.order_id, "order_1");

    // Check if department_tasks contains the CS task
    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM department_tasks WHERE department = 'customer_success' AND event_type = 'RouteStatusUpdated'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(task_count, 1);
}

}