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
        if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            bench_queue("Postgres", pg_queue).await;
        } else {
             println!("Skipping Postgres bench due to connection failure");
        }
    } else {
         println!("Skipping Postgres bench due to missing DATABASE_URL");
    }

    // 2. Standalone Mode - Memory
    println!("--- Standalone Mode (Memory) ---");
    let mem_queue = Arc::new(MemoryTaskQueue::new());
    bench_queue("Memory", mem_queue).await;
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
            let mut job = Job {
                id: format!("job_{}_{}_{}", name, run_id, i),
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
}
