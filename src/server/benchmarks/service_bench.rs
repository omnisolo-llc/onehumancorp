use std::time::Instant;
use std::sync::Arc;
use tonic::Request;
use crate::services::org::service::MyOrgService;
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::org_service_server::OrgService;
use ::server_ohc::orchestration::agent_manager_service_server::AgentManagerService;
use crate::db::{DB, DbStore};
use crate::hub::Hub;

pub async fn bench_org_service_caching() {
    println!("Benchmarking OrgService Caching...");

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(sqlite_pool) });
    let hub = Arc::new(Hub::new(tx, db.pool.clone()));

    let service = MyOrgService::new(hub);

    let iterations = 1000;

    // 1. GetDomains
    println!("--- GetDomains ---");
    {
        // Warm up
        let _ = service.get_domains(Request::new(EmptyRequest { mobile_optimized: false })).await;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = service.get_domains(Request::new(EmptyRequest { mobile_optimized: false })).await;
        }
        let elapsed = start.elapsed();
        println!("GetDomains (Cached) x {}: {:?}, avg: {} ns", iterations, elapsed, elapsed.as_nanos() / iterations as u128);
    }

    // 2. GetMarketplaceItems
    println!("--- GetMarketplaceItems ---");
    {
        // Warm up
        let _ = service.get_marketplace_items(Request::new(EmptyRequest { mobile_optimized: false })).await;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = service.get_marketplace_items(Request::new(EmptyRequest { mobile_optimized: false })).await;
        }
        let elapsed = start.elapsed();
        println!("GetMarketplaceItems (Cached) x {}: {:?}, avg: {} ns", iterations, elapsed, elapsed.as_nanos() / iterations as u128);
    }

    // 3. GetAnalytics (mostly cached parts)
    println!("--- GetAnalytics ---");
    {
        let mut req = Request::new(EmptyRequest { mobile_optimized: false });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://ohc/org/system/agent/test".parse().unwrap());

        // Warm up
        let _ = service.get_analytics(req).await;

        let start = Instant::now();
        for _ in 0..iterations {
            let mut req = Request::new(EmptyRequest { mobile_optimized: false });
            req.metadata_mut().insert("x-spiffe-id", "spiffe://ohc/org/system/agent/test".parse().unwrap());
            let _ = service.get_analytics(req).await;
        }
        let elapsed = start.elapsed();
        println!("GetAnalytics (Cached) x {}: {:?}, avg: {} ns", iterations, elapsed, elapsed.as_nanos() / iterations as u128);
    }
}

pub async fn bench_mobile_payload_size() {
    println!("Benchmarking Mobile Payload Optimization...");

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(sqlite_pool) });
    let hub = Arc::new(Hub::new(tx, db.pool.clone()));

    // Seed some data for snapshot
    for i in 0..100 {
        hub.register_agent(Agent {
            id: format!("agent-{}", i),
            name: "A very long name that should be cleared in mobile optimization to save bandwidth and tokens".to_string(),
            role: "worker".to_string(),
            organization_id: "system".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
    }

    let agent_service = crate::services::agent::service::MyAgentManagerService::new(hub.clone());

    let mut req_desktop = Request::new(EmptyRequest { mobile_optimized: false });
    req_desktop.metadata_mut().insert("x-spiffe-id", "spiffe://ohc/org/system/agent/test".parse().unwrap());
    req_desktop.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://ohc/org/system/agent/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });
    let res_desktop = agent_service.get_dashboard_snapshot(req_desktop).await.unwrap().into_inner();

    let mut req_mobile = Request::new(EmptyRequest { mobile_optimized: true });
    req_mobile.metadata_mut().insert("x-spiffe-id", "spiffe://ohc/org/system/agent/test".parse().unwrap());
    req_mobile.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://ohc/org/system/agent/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });
    let res_mobile = agent_service.get_dashboard_snapshot(req_mobile).await.unwrap().into_inner();

    let desktop_size = serde_json::to_string(&res_desktop).unwrap().len();
    let mobile_size = serde_json::to_string(&res_mobile).unwrap().len();

    println!("DashboardSnapshot Desktop size: {} bytes", desktop_size);
    println!("DashboardSnapshot Mobile size: {} bytes", mobile_size);
    println!("Reduction: {:.2}%", (1.0 - (mobile_size as f64 / desktop_size as f64)) * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_org_service() {
        bench_org_service_caching().await;
    }

    #[tokio::test]
    async fn test_bench_mobile_payload() {
        bench_mobile_payload_size().await;
    }
}
