use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

// Phase 1: Baseline / Phase 2: Parallel Fetching Optimization & Batching
pub async fn bench_queue_latency() {
    println!("Benchmarking Latency...");

    // 1. Cloud Mode - Postgres
    println!("--- Cloud Mode (Postgres) ---");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url != "postgres://localhost/dummy" {
        let pool_res = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .connect(&database_url).await;

        if let Ok(pg_pool) = pool_res {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            bench_queue("Postgres", pg_queue).await;
        } else {

        }
    } else {

    }

    // 2. Standalone Mode - Memory
    println!("--- Standalone Mode (Memory) ---");
    let mem_queue = Arc::new(MemoryTaskQueue::new());
    bench_queue("Memory", mem_queue).await;
}

pub async fn bench_dashboard_snapshot() {
    println!("Benchmarking Dashboard Snapshot Fetching...");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url == "postgres://localhost/dummy" {

        return;
    }

    let pool_res = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
        .connect(&database_url).await;

    let pg_pool = match pool_res {
        Ok(p) => p,
        Err(_e) => {
            return;
        }
    };

    let hub = Arc::new(crate::hub::Hub::new(tx, pg_pool));

    let iterations = 100;
    let mut fetch_times = Vec::new();

    for i in 0..50 {
        hub.register_agent(crate::ohc::orchestration::Agent {
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

        let (agents_res, meetings_res, cost_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            })
        );
        let _ = agents_res.unwrap_or_default();
        let _ = meetings_res.unwrap_or_default();
        let _ = cost_res.unwrap_or_default();

        fetch_times.push(start.elapsed().as_micros());
    }

    fetch_times.sort();
    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);
}

// Emulating high-concurrency dispatch scenarios (Phase 2 Parallel Execution Strategy)
async fn bench_queue(name: &str, queue: Arc<dyn TaskQueue>) {
    let mut enqueue_times = Vec::new();
    let mut dequeue_times = Vec::new();
    let iterations = 100;

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
            q.enqueue(job).await.unwrap();
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

    let enq_p50 = enqueue_times[iterations / 2];
    let enq_p95 = enqueue_times[(iterations as f32 * 0.95) as usize];
    let enq_p99 = enqueue_times[(iterations as f32 * 0.99) as usize];

    let deq_p50 = dequeue_times[iterations / 2];
    let deq_p95 = dequeue_times[(iterations as f32 * 0.95) as usize];
    let deq_p99 = dequeue_times[(iterations as f32 * 0.99) as usize];

    println!("{}: Enqueue p50: {} us, p95: {} us, p99: {} us", name, enq_p50, enq_p95, enq_p99);
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
    async fn test_bench_dashboard_snapshot() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        if db_url == "dummy" || !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await, Ok(Ok(_))) {
            return;
        }
        bench_dashboard_snapshot().await;
    }
}
