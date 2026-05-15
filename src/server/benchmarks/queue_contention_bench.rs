use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

pub async fn bench_queue_contention() {
    tracing::info!("Benchmarking Queue Contention (Memory vs Postgres)...");

    let iterations = 5000;
    let concurrency = 100;

    let mem_queue = Arc::new(MemoryTaskQueue::new());
    run_contention_benchmark("Memory Queue", mem_queue, iterations, concurrency).await;

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url.starts_with("postgres") && database_url != "postgres://localhost/dummy" {
        if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new()
            .max_connections(50)
            .connect(&database_url).await
        {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            run_contention_benchmark("Postgres Queue", pg_queue, iterations, concurrency).await;
        }
    }
}

async fn run_contention_benchmark(name: &str, queue: Arc<dyn TaskQueue>, total_jobs: usize, concurrency: usize) {
    let mut enqueue_join_handles = Vec::new();
    let jobs_per_worker = total_jobs / concurrency;

    let start_enqueue = Instant::now();
    for worker_id in 0..concurrency {
        let q = queue.clone();
        let name = name.to_string();

        enqueue_join_handles.push(tokio::spawn(async move {
            let mut batch = Vec::new();
            for i in 0..jobs_per_worker {
                batch.push(Job {
                    id: format!("job_{}_{}_{}_{}", name, worker_id, i, Uuid::new_v4()),
                    tenant_id: "contention_tenant".to_string(),
                    parent_task_id: "".to_string(),
                    agent_role: "contention_agent".to_string(),
                    payload: r#"{"type": "contention_test", "data": "large_payload_string_here_to_simulate_workload"}"#.to_string(),
                    status: "PENDING".to_string(),
                    attempts: 0,
                    max_attempts: 3,
                    run_after: Utc::now(),
                    locked_until: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                });

                if batch.len() >= 50 {
                    q.enqueue_batch(batch.clone()).await.unwrap();
                    batch.clear();
                }
            }
            if !batch.is_empty() {
                q.enqueue_batch(batch).await.unwrap();
            }
        }));
    }

    for handle in enqueue_join_handles {
        let _ = handle.await;
    }
    let enqueue_duration = start_enqueue.elapsed();
    println!("{}: Enqueued {} jobs across {} workers in {:?}", name, total_jobs, concurrency, enqueue_duration);

    let mut dequeue_join_handles = Vec::new();
    let start_dequeue = Instant::now();

    let jobs_dequeued = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    for _worker_id in 0..concurrency {
        let q = queue.clone();
        let jobs_dequeued_clone = jobs_dequeued.clone();

        dequeue_join_handles.push(tokio::spawn(async move {
            loop {
                if let Ok(Some(job)) = q.dequeue(vec!["contention_agent".to_string()]).await {
                    let _ = q.complete(&job.id, &job.tenant_id).await;
                    jobs_dequeued_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    break;
                }
            }
        }));
    }

    for handle in dequeue_join_handles {
        let _ = handle.await;
    }

    let dequeue_duration = start_dequeue.elapsed();
    let final_dequeued = jobs_dequeued.load(std::sync::atomic::Ordering::Relaxed);
    println!("{}: Dequeued and completed {} jobs across {} workers in {:?}", name, final_dequeued, concurrency, dequeue_duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_queue_contention() {
        bench_queue_contention().await;
    }
}
