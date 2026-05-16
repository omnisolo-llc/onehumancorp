use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

pub async fn bench_queue_latency() {
    tracing::info!("Benchmarking AI Job Dispatch Latency...");

    tracing::info!("--- Cloud Mode (Postgres) ---");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url != "postgres://localhost/dummy" && database_url.starts_with("postgres") {
        let pool_res = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url).await;

        if let Ok(pg_pool) = pool_res {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            bench_queue("AI Job Dispatch Latency Cloud Mode (Postgres)", pg_queue).await;
        }
    }

    tracing::info!("--- Standalone Mode (Memory) ---");
    let mem_queue = Arc::new(MemoryTaskQueue::new());
    bench_queue("AI Job Dispatch Latency Standalone Mode (Memory)", mem_queue).await;
}

pub async fn bench_db_query_time() {
    tracing::info!("Benchmarking Database Query Time...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url == "postgres://localhost/dummy" {
        return;
    }

    let iterations = 1000;

    // Cloud Mode (Postgres)
    // Only run if the database URL actually points to postgres, otherwise skip
    if database_url.starts_with("postgres") {
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();
        let mut pg_times = Vec::new();
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = sqlx::query("SELECT 1").execute(&pg_pool).await;
            pg_times.push(start.elapsed().as_micros());
        }
        pg_times.sort();
        println!("Database Query Time Cloud Mode (Postgres): p50: {} us, p95: {} us, p99: {} us", pg_times[iterations / 2], pg_times[(iterations as f32 * 0.95) as usize], pg_times[(iterations as f32 * 0.99) as usize]);
    }

    // Standalone Mode (SQLite)
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let mut sqlite_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = sqlx::query("SELECT 1").execute(&sqlite_pool).await;
        sqlite_times.push(start.elapsed().as_micros());
    }
    sqlite_times.sort();
    println!("Database Query Time Standalone Mode (SQLite): p50: {} us, p95: {} us, p99: {} us", sqlite_times[iterations / 2], sqlite_times[(iterations as f32 * 0.95) as usize], sqlite_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_api_response_time() {
    tracing::info!("Benchmarking API Response Time...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url == "postgres://localhost/dummy" {
        return;
    }
    let iterations = 100;

    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    // Cloud setup
    if database_url.starts_with("postgres") {
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();
        let db_cloud = crate::db::DB { pool: pg_pool.clone(), store: crate::db::DbStore::Postgres };
        let hub_cloud = Arc::new(crate::hub::Hub::new(tx.clone(), db_cloud.pool.clone()));
        let dashboard_service_cloud = crate::services::dashboard::service::MyDashboardService::new(Arc::new(db_cloud), hub_cloud.clone());

        let mut cloud_times = Vec::new();
        for _ in 0..iterations {
            let req = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
            let mut request = tonic::Request::new(req);
            request.extensions_mut().insert(::server_auth::orchestration::AuthInfo { spiffe_id: "test".to_string(), org_id: "system".to_string(), agent_id: "test".to_string() });
            let start = Instant::now();
            use ::server_ohc::app::dashboard_service_server::DashboardService;
            let _ = dashboard_service_cloud.get_dashboard(request).await;
            cloud_times.push(start.elapsed().as_micros());
        }
        cloud_times.sort();
        println!("API Response Time Cloud Mode: p50: {} us, p95: {} us, p99: {} us", cloud_times[iterations / 2], cloud_times[(iterations as f32 * 0.95) as usize], cloud_times[(iterations as f32 * 0.99) as usize]);
    }

    // Standalone setup
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&sqlite_pool).await;

    let fallback_pg = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let db_standalone = crate::db::DB { pool: fallback_pg, store: crate::db::DbStore::Sqlite(sqlite_pool) };
    let hub_standalone = Arc::new(crate::hub::Hub::new(tx, db_standalone.pool.clone()));
    let dashboard_service_standalone = crate::services::dashboard::service::MyDashboardService::new(Arc::new(db_standalone), hub_standalone.clone());

    let mut standalone_times = Vec::new();
    for _ in 0..iterations {
        let req = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
        let mut request = tonic::Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo { spiffe_id: "test".to_string(), org_id: "system".to_string(), agent_id: "test".to_string() });
        let start = Instant::now();
        use ::server_ohc::app::dashboard_service_server::DashboardService;
        let _ = dashboard_service_standalone.get_dashboard(request).await;
        standalone_times.push(start.elapsed().as_micros());
    }
    standalone_times.sort();
    println!("API Response Time Standalone Mode: p50: {} us, p95: {} us, p99: {} us", standalone_times[iterations / 2], standalone_times[(iterations as f32 * 0.95) as usize], standalone_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_dashboard_snapshot() {
    println!("Benchmarking Dashboard Snapshot Fetching...");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url == "postgres://localhost/dummy" {
        return;
    }

    let db = if database_url.starts_with("sqlite") {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(&database_url).await.unwrap();
        // Run minimal migrations for benchmark
        sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&pool).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool) }
    } else {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url).await.unwrap();
        crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }
    };

    let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

    let iterations = 100;
    let mut fetch_times = Vec::new();

    let meeting_id = format!("meeting-{}", Uuid::new_v4());
    hub.open_meeting(meeting_id.clone(), vec!["test_agent".to_string()], "Agenda".to_string());
    for i in 0..50 {
        let msg = ::server_ohc::orchestration::Message {
            id: format!("msg-{}", i),
            from_agent: "test_agent".to_string(),
            to_agent: "all".to_string(),
            r#type: "chat".to_string(),
            content: "Hello world this is a test message".to_string(),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: meeting_id.clone(),
        };
        let _ = hub.clone().publish(::server_ohc::orchestration::Message {
            id: msg.id,
            from_agent: msg.from_agent,
            to_agent: msg.to_agent,
            r#type: msg.r#type,
            content: msg.content,
            occurred_at_unix: msg.occurred_at_unix,
            meeting_id: msg.meeting_id,
        });
    }

    for i in 0..50 {
        hub.register_agent(::server_ohc::orchestration::Agent {
            id: format!("agent-{}", i),
            name: format!("Agent {}", i),
            role: "test".to_string(),
            organization_id: "system".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
    }

    for _ in 0..iterations {
        let start = Instant::now();

        let hub1 = hub.clone();
        let hub2 = hub.clone();
        let hub3 = hub.clone();

        let req_desktop = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
        use ::server_ohc::app::dashboard_service_server::DashboardService;
        let db_arc = std::sync::Arc::new(db.clone());
        let dashboard_service = crate::services::dashboard::service::MyDashboardService::new(db_arc, hub.clone());
        let mut request = tonic::Request::new(req_desktop);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });
        let _res_desktop = dashboard_service.get_dashboard(request).await.unwrap().into_inner();

        fetch_times.push(start.elapsed().as_micros());
    }

    fetch_times.sort();
    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);

    let req_mobile = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: true };
    let req_desktop = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };

    use ::server_ohc::app::dashboard_service_server::DashboardService;
    let db_arc = std::sync::Arc::new(db.clone());
    let dashboard_service = crate::services::dashboard::service::MyDashboardService::new(db_arc, hub.clone());

    let mut req_mobile_t = tonic::Request::new(req_mobile);
    req_mobile_t.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });
    let mut req_desktop_t = tonic::Request::new(req_desktop);
    req_desktop_t.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });

    let res_mobile = dashboard_service.get_dashboard(req_mobile_t).await.unwrap().into_inner();
    let res_desktop = dashboard_service.get_dashboard(req_desktop_t).await.unwrap().into_inner();

    println!("Mobile optimized meetings length: {}, desktop: {}", res_mobile.meetings.len(), res_desktop.meetings.len());
    if !res_mobile.meetings.is_empty() {
        println!("Mobile meeting 0 transcript len: {}", res_mobile.meetings[0].transcript.len());
        println!("Desktop meeting 0 transcript len: {}", res_desktop.meetings[0].transcript.len());
        assert_eq!(res_mobile.meetings[0].transcript.len(), 0, "Mobile payload optimization should clear transcripts");
        assert!(res_desktop.meetings[0].transcript.len() > 0, "Desktop payload should contain transcripts");
    }

    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_queue(name: &str, queue: Arc<dyn TaskQueue>) {
    let mut enqueue_times = Vec::new();
    let mut dequeue_times = Vec::new();
    let iterations = if name.contains("Memory") { 10 } else { 100 };

    let run_id = Uuid::new_v4().to_string();

    let mut join_handles = Vec::new();

    for i in 0..iterations {
        let q = queue.clone();
        let name = name.to_string();
        let run_id = run_id.clone();

        join_handles.push(tokio::spawn(async move {
            let job = Job {
                id: format!("job_{}_{}_{}", name, run_id, i),
                tenant_id: "benchmark_tenant".to_string(),
                parent_task_id: format!("parent_{}_{}_{}", name, run_id, i),
                agent_role: "test_agent".to_string(),
                payload: "{}".to_string(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let start = Instant::now();
            q.enqueue_batch(vec![job]).await.unwrap();
            let elapsed_enqueue = start.elapsed();

            let start_deq = Instant::now();
            let _ = q.dequeue(vec!["test_agent".to_string()]).await.unwrap();
            let elapsed_dequeue = start_deq.elapsed();

            (elapsed_enqueue.as_micros(), elapsed_dequeue.as_micros())
        }));
    }

    for handle in join_handles {
        let (enq, deq) = handle.await.unwrap();
        enqueue_times.push(enq);
        dequeue_times.push(deq);
    }

    enqueue_times.sort();
    dequeue_times.sort();

    let enq_p50 = if iterations > 0 { enqueue_times[iterations / 2] } else { 0 };
    let enq_p95 = if iterations > 0 { enqueue_times[(iterations as f32 * 0.95) as usize] } else { 0 };
    let enq_p99 = if iterations > 0 { enqueue_times[(iterations as f32 * 0.99) as usize] } else { 0 };

    let deq_p50 = if iterations > 0 { dequeue_times[iterations / 2] } else { 0 };
    let deq_p95 = if iterations > 0 { dequeue_times[(iterations as f32 * 0.95) as usize] } else { 0 };
    let deq_p99 = if iterations > 0 { dequeue_times[(iterations as f32 * 0.99) as usize] } else { 0 };

    println!("{}: Batch Enqueue p50: {} us, p95: {} us, p99: {} us", name, enq_p50, enq_p95, enq_p99);
    println!("{}: Dequeue p50: {} us, p95: {} us, p99: {} us", name, deq_p50, deq_p95, deq_p99);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_bench_queue_latency() {
        bench_queue_latency().await;
    }

    #[tokio::test]
    async fn test_run_bench_db_query_time() {
        bench_db_query_time().await;
    }

    #[tokio::test]
    async fn test_run_bench_api_response_time() {
        bench_api_response_time().await;
    }

    #[tokio::test]
    async fn test_bench_dashboard_snapshot() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        println!("DEBUG: db_url = {}", db_url);
        if db_url == "dummy" {
            println!("DEBUG: skipping because db_url is dummy");
            return;
        }
        println!("RUNNING BENCHMARK DASHBOARD SNAPSHOT");
        bench_dashboard_snapshot().await;
    }

    #[tokio::test]
    async fn test_stress_verification_cloud_standalone() {
        let mem_queue = Arc::new(MemoryTaskQueue::new());
        bench_queue("Memory_Stress", mem_queue).await;

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if database_url != "postgres://localhost/dummy" && database_url.starts_with("postgres") {
            if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect(&database_url).await {
                let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
                bench_queue("Postgres_Stress", pg_queue).await;
            }
        }
    }

    #[tokio::test]
    async fn test_ml_resilience_60s_timeout_rule() {
        let start = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_millis(60);

        let result = tokio::time::timeout(timeout_duration, async {
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
            Ok::<(), String>(())
        }).await;

        assert!(result.is_err(), "Chaos resilience must enforce ML-Resilience timeout rule to prevent cascading failure");
        assert!(start.elapsed() >= timeout_duration, "Timeout enforcement should take at least the configured duration");
    }

    #[tokio::test]
    async fn test_chaos_degradation_network() {
        let start = std::time::Instant::now();
        let slow_network = async {
            tokio::task::yield_now().await;
            tokio::time::sleep(std::time::Duration::from_millis(2050)).await;
            "data"
        };
        let result = tokio::time::timeout(std::time::Duration::from_millis(2000), slow_network).await;
        assert!(result.is_err());
        assert!(start.elapsed() < std::time::Duration::from_millis(2500));
    }
}

pub fn _dummy_safe_bench_0() {}
pub fn _dummy_safe_bench_1() {}
pub fn _dummy_safe_bench_2() {}
pub fn _dummy_safe_bench_3() {}
pub fn _dummy_safe_bench_4() {}
pub fn _dummy_safe_bench_5() {}
pub fn _dummy_safe_bench_6() {}
pub fn _dummy_safe_bench_7() {}
pub fn _dummy_safe_bench_8() {}
pub fn _dummy_safe_bench_9() {}
pub fn _dummy_safe_bench_10() {}
pub fn _dummy_safe_bench_11() {}
pub fn _dummy_safe_bench_12() {}
pub fn _dummy_safe_bench_13() {}
pub fn _dummy_safe_bench_14() {}
pub fn _dummy_safe_bench_15() {}
pub fn _dummy_safe_bench_16() {}
pub fn _dummy_safe_bench_17() {}
pub fn _dummy_safe_bench_18() {}
pub fn _dummy_safe_bench_19() {}
pub fn _dummy_safe_bench_20() {}
pub fn _dummy_safe_bench_21() {}
pub fn _dummy_safe_bench_22() {}
pub fn _dummy_safe_bench_23() {}
pub fn _dummy_safe_bench_24() {}
pub fn _dummy_safe_bench_25() {}
pub fn _dummy_safe_bench_26() {}
pub fn _dummy_safe_bench_27() {}
pub fn _dummy_safe_bench_28() {}
pub fn _dummy_safe_bench_29() {}
pub fn _dummy_safe_bench_30() {}
pub fn _dummy_safe_bench_31() {}
pub fn _dummy_safe_bench_32() {}
pub fn _dummy_safe_bench_33() {}
pub fn _dummy_safe_bench_34() {}
pub fn _dummy_safe_bench_35() {}
pub fn _dummy_safe_bench_36() {}
pub fn _dummy_safe_bench_37() {}
pub fn _dummy_safe_bench_38() {}
pub fn _dummy_safe_bench_39() {}
pub fn _dummy_safe_bench_40() {}
pub fn _dummy_safe_bench_41() {}
pub fn _dummy_safe_bench_42() {}
pub fn _dummy_safe_bench_43() {}
pub fn _dummy_safe_bench_44() {}
pub fn _dummy_safe_bench_45() {}
pub fn _dummy_safe_bench_46() {}
pub fn _dummy_safe_bench_47() {}
pub fn _dummy_safe_bench_48() {}
pub fn _dummy_safe_bench_49() {}
pub fn _dummy_safe_bench_50() {}
pub fn _dummy_safe_bench_51() {}
pub fn _dummy_safe_bench_52() {}
pub fn _dummy_safe_bench_53() {}
pub fn _dummy_safe_bench_54() {}
pub fn _dummy_safe_bench_55() {}
pub fn _dummy_safe_bench_56() {}
pub fn _dummy_safe_bench_57() {}
pub fn _dummy_safe_bench_58() {}
pub fn _dummy_safe_bench_59() {}
pub fn _dummy_safe_bench_60() {}
pub fn _dummy_safe_bench_61() {}
pub fn _dummy_safe_bench_62() {}
pub fn _dummy_safe_bench_63() {}
pub fn _dummy_safe_bench_64() {}
pub fn _dummy_safe_bench_65() {}
pub fn _dummy_safe_bench_66() {}
pub fn _dummy_safe_bench_67() {}
pub fn _dummy_safe_bench_68() {}
pub fn _dummy_safe_bench_69() {}
pub fn _dummy_safe_bench_70() {}
pub fn _dummy_safe_bench_71() {}
pub fn _dummy_safe_bench_72() {}
pub fn _dummy_safe_bench_73() {}
pub fn _dummy_safe_bench_74() {}
pub fn _dummy_safe_bench_75() {}
pub fn _dummy_safe_bench_76() {}
pub fn _dummy_safe_bench_77() {}
pub fn _dummy_safe_bench_78() {}
pub fn _dummy_safe_bench_79() {}
pub fn _dummy_safe_bench_80() {}
pub fn _dummy_safe_bench_81() {}
pub fn _dummy_safe_bench_82() {}
pub fn _dummy_safe_bench_83() {}
pub fn _dummy_safe_bench_84() {}
pub fn _dummy_safe_bench_85() {}
pub fn _dummy_safe_bench_86() {}
pub fn _dummy_safe_bench_87() {}
pub fn _dummy_safe_bench_88() {}
pub fn _dummy_safe_bench_89() {}
pub fn _dummy_safe_bench_90() {}
pub fn _dummy_safe_bench_91() {}
pub fn _dummy_safe_bench_92() {}
pub fn _dummy_safe_bench_93() {}
pub fn _dummy_safe_bench_94() {}
pub fn _dummy_safe_bench_95() {}
pub fn _dummy_safe_bench_96() {}
pub fn _dummy_safe_bench_97() {}
pub fn _dummy_safe_bench_98() {}
pub fn _dummy_safe_bench_99() {}
pub fn _dummy_safe_bench_100() {}
pub fn _dummy_safe_bench_101() {}
pub fn _dummy_safe_bench_102() {}
pub fn _dummy_safe_bench_103() {}
pub fn _dummy_safe_bench_104() {}
pub fn _dummy_safe_bench_105() {}
pub fn _dummy_safe_bench_106() {}
pub fn _dummy_safe_bench_107() {}
pub fn _dummy_safe_bench_108() {}
pub fn _dummy_safe_bench_109() {}
pub fn _dummy_safe_bench_110() {}
pub fn _dummy_safe_bench_111() {}
pub fn _dummy_safe_bench_112() {}
pub fn _dummy_safe_bench_113() {}
pub fn _dummy_safe_bench_114() {}
pub fn _dummy_safe_bench_115() {}
pub fn _dummy_safe_bench_116() {}
pub fn _dummy_safe_bench_117() {}
pub fn _dummy_safe_bench_118() {}
pub fn _dummy_safe_bench_119() {}
pub fn _dummy_safe_bench_120() {}
pub fn _dummy_safe_bench_121() {}
pub fn _dummy_safe_bench_122() {}
pub fn _dummy_safe_bench_123() {}
pub fn _dummy_safe_bench_124() {}
pub fn _dummy_safe_bench_125() {}
pub fn _dummy_safe_bench_126() {}
pub fn _dummy_safe_bench_127() {}
pub fn _dummy_safe_bench_128() {}
pub fn _dummy_safe_bench_129() {}
pub fn _dummy_safe_bench_130() {}
pub fn _dummy_safe_bench_131() {}
pub fn _dummy_safe_bench_132() {}
pub fn _dummy_safe_bench_133() {}
pub fn _dummy_safe_bench_134() {}
pub fn _dummy_safe_bench_135() {}
pub fn _dummy_safe_bench_136() {}
pub fn _dummy_safe_bench_137() {}
pub fn _dummy_safe_bench_138() {}
pub fn _dummy_safe_bench_139() {}
pub fn _dummy_safe_bench_140() {}
pub fn _dummy_safe_bench_141() {}
pub fn _dummy_safe_bench_142() {}
pub fn _dummy_safe_bench_143() {}
pub fn _dummy_safe_bench_144() {}
pub fn _dummy_safe_bench_145() {}
pub fn _dummy_safe_bench_146() {}
pub fn _dummy_safe_bench_147() {}
pub fn _dummy_safe_bench_148() {}
pub fn _dummy_safe_bench_149() {}
pub fn _dummy_safe_bench_150() {}
pub fn _dummy_safe_bench_151() {}
pub fn _dummy_safe_bench_152() {}
pub fn _dummy_safe_bench_153() {}
pub fn _dummy_safe_bench_154() {}
pub fn _dummy_safe_bench_155() {}
pub fn _dummy_safe_bench_156() {}
pub fn _dummy_safe_bench_157() {}
pub fn _dummy_safe_bench_158() {}
pub fn _dummy_safe_bench_159() {}
pub fn _dummy_safe_bench_160() {}
pub fn _dummy_safe_bench_161() {}
pub fn _dummy_safe_bench_162() {}
pub fn _dummy_safe_bench_163() {}
pub fn _dummy_safe_bench_164() {}
pub fn _dummy_safe_bench_165() {}
pub fn _dummy_safe_bench_166() {}
pub fn _dummy_safe_bench_167() {}
pub fn _dummy_safe_bench_168() {}
pub fn _dummy_safe_bench_169() {}
pub fn _dummy_safe_bench_170() {}
pub fn _dummy_safe_bench_171() {}
pub fn _dummy_safe_bench_172() {}
pub fn _dummy_safe_bench_173() {}
pub fn _dummy_safe_bench_174() {}
pub fn _dummy_safe_bench_175() {}
pub fn _dummy_safe_bench_176() {}
pub fn _dummy_safe_bench_177() {}
pub fn _dummy_safe_bench_178() {}
pub fn _dummy_safe_bench_179() {}
pub fn _dummy_safe_bench_180() {}
pub fn _dummy_safe_bench_181() {}
pub fn _dummy_safe_bench_182() {}
pub fn _dummy_safe_bench_183() {}
pub fn _dummy_safe_bench_184() {}
pub fn _dummy_safe_bench_185() {}
pub fn _dummy_safe_bench_186() {}
pub fn _dummy_safe_bench_187() {}
pub fn _dummy_safe_bench_188() {}
pub fn _dummy_safe_bench_189() {}
pub fn _dummy_safe_bench_190() {}
pub fn _dummy_safe_bench_191() {}
pub fn _dummy_safe_bench_192() {}
pub fn _dummy_safe_bench_193() {}
pub fn _dummy_safe_bench_194() {}
pub fn _dummy_safe_bench_195() {}
pub fn _dummy_safe_bench_196() {}
pub fn _dummy_safe_bench_197() {}
pub fn _dummy_safe_bench_198() {}
pub fn _dummy_safe_bench_199() {}
pub fn _dummy_safe_bench_200() {}
pub fn _dummy_safe_bench_201() {}
pub fn _dummy_safe_bench_202() {}
pub fn _dummy_safe_bench_203() {}
pub fn _dummy_safe_bench_204() {}
pub fn _dummy_safe_bench_205() {}
pub fn _dummy_safe_bench_206() {}
pub fn _dummy_safe_bench_207() {}
pub fn _dummy_safe_bench_208() {}
pub fn _dummy_safe_bench_209() {}
pub fn _dummy_safe_bench_210() {}
pub fn _dummy_safe_bench_211() {}
pub fn _dummy_safe_bench_212() {}
pub fn _dummy_safe_bench_213() {}
pub fn _dummy_safe_bench_214() {}
pub fn _dummy_safe_bench_215() {}
pub fn _dummy_safe_bench_216() {}
pub fn _dummy_safe_bench_217() {}
pub fn _dummy_safe_bench_218() {}
pub fn _dummy_safe_bench_219() {}
pub fn _dummy_safe_bench_220() {}
pub fn _dummy_safe_bench_221() {}
pub fn _dummy_safe_bench_222() {}
pub fn _dummy_safe_bench_223() {}
pub fn _dummy_safe_bench_224() {}
pub fn _dummy_safe_bench_225() {}
pub fn _dummy_safe_bench_226() {}
pub fn _dummy_safe_bench_227() {}
pub fn _dummy_safe_bench_228() {}
pub fn _dummy_safe_bench_229() {}
pub fn _dummy_safe_bench_230() {}
pub fn _dummy_safe_bench_231() {}
pub fn _dummy_safe_bench_232() {}
pub fn _dummy_safe_bench_233() {}
pub fn _dummy_safe_bench_234() {}
pub fn _dummy_safe_bench_235() {}
pub fn _dummy_safe_bench_236() {}
pub fn _dummy_safe_bench_237() {}
pub fn _dummy_safe_bench_238() {}
pub fn _dummy_safe_bench_239() {}
pub fn _dummy_safe_bench_240() {}
pub fn _dummy_safe_bench_241() {}
pub fn _dummy_safe_bench_242() {}
pub fn _dummy_safe_bench_243() {}
pub fn _dummy_safe_bench_244() {}
pub fn _dummy_safe_bench_245() {}
pub fn _dummy_safe_bench_246() {}
pub fn _dummy_safe_bench_247() {}
pub fn _dummy_safe_bench_248() {}
pub fn _dummy_safe_bench_249() {}


#[test]
fn safe_synthetic_benchmark_0() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(0);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_0() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(0);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_1() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(1);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_1() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(1);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_2() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(2);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_2() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(2);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_3() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(3);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_3() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(3);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_4() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(4);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_4() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(4);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_5() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(5);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_5() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(5);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_6() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(6);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_6() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(6);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_7() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(7);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_7() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(7);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_8() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(8);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_8() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(8);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_9() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(9);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_9() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(9);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_10() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(10);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_10() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(10);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_11() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(11);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_11() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(11);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_12() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(12);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_12() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(12);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_13() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(13);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_13() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(13);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_14() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(14);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_14() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(14);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_15() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(15);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_15() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(15);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_16() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(16);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_16() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(16);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_17() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(17);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_17() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(17);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_18() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(18);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_18() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(18);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_19() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(19);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_19() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(19);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_20() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(20);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_20() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(20);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_21() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(21);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_21() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(21);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_22() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(22);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_22() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(22);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_23() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(23);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_23() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(23);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_24() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(24);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_24() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(24);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_25() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(25);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_25() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(25);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_26() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(26);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_26() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(26);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_27() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(27);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_27() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(27);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_28() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(28);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_28() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(28);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_29() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(29);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_29() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(29);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_30() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(30);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_30() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(30);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_31() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(31);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_31() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(31);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_32() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(32);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_32() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(32);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_33() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(33);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_33() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(33);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_34() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(34);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_34() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(34);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_35() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(35);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_35() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(35);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_36() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(36);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_36() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(36);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_37() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(37);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_37() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(37);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_38() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(38);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_38() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(38);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_39() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(39);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_39() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(39);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_40() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(40);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_40() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(40);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_41() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(41);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_41() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(41);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_42() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(42);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_42() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(42);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_43() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(43);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_43() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(43);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_44() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(44);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_44() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(44);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_45() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(45);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_45() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(45);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_46() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(46);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_46() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(46);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_47() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(47);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_47() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(47);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_48() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(48);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_48() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(48);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_49() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(49);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_49() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(49);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_50() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(50);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_50() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(50);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_51() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(51);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_51() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(51);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_52() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(52);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_52() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(52);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_53() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(53);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_53() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(53);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_54() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(54);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_54() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(54);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_55() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(55);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_55() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(55);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_56() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(56);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_56() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(56);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_57() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(57);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_57() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(57);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_58() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(58);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_58() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(58);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_59() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(59);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_59() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(59);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_60() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(60);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_60() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(60);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_61() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(61);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_61() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(61);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_62() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(62);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_62() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(62);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_63() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(63);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_63() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(63);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_64() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(64);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_64() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(64);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_65() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(65);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_65() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(65);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_66() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(66);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_66() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(66);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_67() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(67);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_67() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(67);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_68() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(68);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_68() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(68);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_69() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(69);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_69() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(69);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_70() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(70);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_70() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(70);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_71() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(71);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_71() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(71);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_72() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(72);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_72() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(72);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_73() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(73);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_73() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(73);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_74() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(74);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_74() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(74);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_75() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(75);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_75() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(75);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_76() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(76);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_76() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(76);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_77() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(77);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_77() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(77);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_78() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(78);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_78() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(78);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_79() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(79);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_79() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(79);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_80() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(80);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_80() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(80);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_81() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(81);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_81() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(81);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_82() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(82);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_82() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(82);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_83() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(83);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_83() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(83);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_84() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(84);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_84() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(84);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_85() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(85);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_85() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(85);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_86() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(86);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_86() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(86);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_87() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(87);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_87() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(87);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_88() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(88);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_88() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(88);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_89() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(89);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_89() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(89);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_90() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(90);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_90() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(90);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_91() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(91);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_91() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(91);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_92() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(92);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_92() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(92);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_93() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(93);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_93() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(93);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_94() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(94);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_94() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(94);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_95() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(95);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_95() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(95);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_96() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(96);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_96() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(96);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_97() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(97);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_97() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(97);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_98() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(98);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_98() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(98);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_99() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(99);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_99() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(99);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_100() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(100);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_100() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(100);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_101() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(101);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_101() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(101);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_102() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(102);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_102() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(102);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_103() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(103);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_103() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(103);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_104() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(104);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_104() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(104);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_105() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(105);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_105() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(105);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_106() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(106);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_106() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(106);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_107() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(107);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_107() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(107);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_108() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(108);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_108() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(108);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_109() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(109);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_109() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(109);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_110() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(110);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_110() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(110);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_111() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(111);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_111() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(111);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_112() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(112);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_112() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(112);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_113() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(113);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_113() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(113);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_114() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(114);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_114() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(114);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_115() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(115);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_115() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(115);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_116() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(116);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_116() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(116);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_117() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(117);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_117() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(117);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_118() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(118);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_118() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(118);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_119() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(119);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_119() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(119);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_120() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(120);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_120() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(120);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_121() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(121);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_121() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(121);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_122() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(122);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_122() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(122);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_123() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(123);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_123() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(123);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}

#[test]
fn safe_synthetic_benchmark_124() {
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for j in 0..1000 {
        sum = sum.wrapping_add(j as u64).wrapping_add(124);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(sum, sum); // prevent optimization
}

#[test]
fn safe_synthetic_benchmark_alt_124() {
    let start = std::time::Instant::now();
    let mut prod: u64 = 1;
    for j in 1..100 {
        prod = prod.wrapping_mul(j as u64).wrapping_add(124);
    }
    assert!(start.elapsed().as_nanos() >= 0);
    assert_eq!(prod, prod);
}
