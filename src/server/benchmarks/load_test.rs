use std::sync::Arc;
use tokio::sync::mpsc;
use std::time::{Duration, Instant};
use crate::hub::Hub;
use crate::db::{DB, DbStore};
use crate::services::dashboard::service::MyDashboardService;
use ::server_ohc::app::*;
use ::server_ohc::app::dashboard_service_server::DashboardService;
use tonic::Request;
use crate::queue::TaskQueue;

pub struct LoadTester {
    hub: Arc<Hub>,
    db: Arc<DB>,
}

impl LoadTester {
    pub fn new(hub: Arc<Hub>, db: Arc<DB>) -> Self {
        Self { hub, db }
    }

    pub async fn run_dashboard_load_test(&self, concurrent_users: usize, duration_secs: u64) {
        println!("Starting Dashboard Load Test: {} concurrent users, {} seconds", concurrent_users, duration_secs);

        let service = Arc::new(MyDashboardService::new(self.db.clone(), self.hub.clone()));
        let start_time = Instant::now();
        let end_time = start_time + Duration::from_secs(duration_secs);

        let (tx, mut rx) = mpsc::channel::<(bool, Duration)>(concurrent_users * 10);

        let mut handles = vec![];

        for user_id in 0..concurrent_users {
            let svc = service.clone();
            let tx_clone = tx.clone();
            let org_id = format!("org-{}", user_id % 10);

            handles.push(tokio::spawn(async move {
                while Instant::now() < end_time {
                    let req = GetDashboardRequest {
                        organization_id: org_id.clone(),
                        mobile_optimized: user_id % 2 == 0,
                    };
                    let mut request = Request::new(req);
                    request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
                        spiffe_id: format!("spiffe://ohc/org/{}/agent/user-{}", org_id, user_id),
                        org_id: org_id.clone(),
                        agent_id: format!("user-{}", user_id),
                    });

                    let start = Instant::now();
                    let res = svc.get_dashboard(request).await;
                    let elapsed = start.elapsed();

                    let _ = tx_clone.send((res.is_ok(), elapsed)).await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }));
        }

        drop(tx);

        let mut total_requests = 0;
        let mut successful_requests = 0;
        let mut latencies = vec![];

        while let Some((success, elapsed)) = rx.recv().await {
            total_requests += 1;
            if success {
                successful_requests += 1;
            }
            latencies.push(elapsed.as_micros());
        }

        latencies.sort();

        if !latencies.is_empty() {
            let p50 = latencies[latencies.len() / 2];
            let p95 = latencies[(latencies.len() as f32 * 0.95) as usize];
            let p99 = latencies[(latencies.len() as f32 * 0.99) as usize];

            println!("Load Test Results:");
            println!("Total Requests: {}", total_requests);
            println!("Success Rate: {:.2}%", (successful_requests as f64 / total_requests as f64) * 100.0);
            println!("Throughput: {:.2} req/s", total_requests as f64 / duration_secs as f64);
            println!("Latency p50: {} us", p50);
            println!("Latency p95: {} us", p95);
            println!("Latency p99: {} us", p99);
        }
    }

    pub async fn run_parallel_job_dispatch_test(&self, total_jobs: usize) {
        println!("Starting Parallel Job Dispatch Test: {} jobs", total_jobs);

        let (_tx, _rx) = mpsc::channel::<serde_json::Value>(100);
        let queue = crate::queue::MemoryTaskQueue::new();

        let start = Instant::now();
        let mut jobs = vec![];
        for i in 0..total_jobs {
            jobs.push(crate::queue::Job {
                id: format!("job-{}", i),
                tenant_id: "system".to_string(),
                parent_task_id: "parent".to_string(),
                agent_role: "worker".to_string(),
                payload: "{}".to_string(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: chrono::Utc::now(),
                locked_until: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            });
        }

        queue.enqueue_batch(jobs).await.unwrap();
        let elapsed = start.elapsed();

        println!("Enqueued {} jobs in {:?}", total_jobs, elapsed);
        println!("Avg enqueue time per job: {:?} ns", elapsed.as_nanos() / total_jobs as u128);

        let start = Instant::now();
        let mut processed = 0;
        while processed < total_jobs {
            if let Some(_) = queue.dequeue(vec!["worker".to_string()]).await.unwrap() {
                processed += 1;
            } else {
                break;
            }
        }
        let elapsed = start.elapsed();
        println!("Dequeued {} jobs in {:?}", total_jobs, elapsed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_tester_smoke() {
        let (tx, _rx) = tokio::sync::mpsc::channel::<serde_json::Value>(100);
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();

        // Setup schema
        sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, name TEXT, description TEXT, price_cents INTEGER, fulfillment_strategy TEXT, currency TEXT, metadata TEXT)").execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&sqlite_pool).await.unwrap();

        let db = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(sqlite_pool) });
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));

        let tester = LoadTester::new(hub, db);
        tester.run_parallel_job_dispatch_test(100).await;
        tester.run_dashboard_load_test(5, 1).await;
    }
}
