use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, PostgresTaskQueue, Job};
use chrono::Utc;
use uuid::Uuid;
use tokio::task::JoinHandle;

pub async fn bench_ai_job_dispatch_latency() {
    tracing::info!("Benchmarking AI Job Dispatch Latency (End-to-End)...");

    let iterations = 100;

    let mem_queue = Arc::new(MemoryTaskQueue::new());
    run_dispatch_benchmark("Standalone Mode (Memory Queue)", mem_queue, iterations).await;

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url.starts_with("postgres") && database_url != "postgres://localhost/dummy" {
        if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url).await
        {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            run_dispatch_benchmark("Cloud Mode (Postgres Queue)", pg_queue, iterations).await;
        }
    }
}

async fn run_dispatch_benchmark(name: &str, queue: Arc<dyn TaskQueue>, total_jobs: usize) {
    let q_producer = queue.clone();
    let q_consumer = queue.clone();

    let run_id = Uuid::new_v4().to_string();
    let name_string = name.to_string();

    let start_e2e = Instant::now();

    let producer_handle = tokio::spawn(async move {
        let mut batch = Vec::new();
        for i in 0..total_jobs {
            batch.push(Job {
                id: format!("job_{}_{}_{}", name_string, run_id, i),
                tenant_id: "latency_tenant".to_string(),
                parent_task_id: "".to_string(),
                agent_role: "latency_agent".to_string(),
                payload: "{}".to_string(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });

            if batch.len() >= 100 {
                q_producer.enqueue_batch(batch.clone()).await.unwrap();
                batch.clear();
            }
        }
        if !batch.is_empty() {
            q_producer.enqueue_batch(batch).await.unwrap();
        }
    });

    let consumer_handle = tokio::spawn(async move {
        let mut jobs_processed = 0;
        let mut empty_polls = 0;

        loop {
            if let Ok(Some(job)) = q_consumer.dequeue(vec!["latency_agent".to_string()]).await {
                let _ = q_consumer.complete(&job.id, &job.tenant_id).await;
                jobs_processed += 1;
                empty_polls = 0;

                if jobs_processed == total_jobs {
                    break;
                }
            } else {
                empty_polls += 1;
                if empty_polls > 100 && jobs_processed == total_jobs {
                    break; // Safety break
                }
                if empty_polls > 500 {
                    println!("Warning: Consumer safety timeout hit with {}/{} jobs processed", jobs_processed, total_jobs);
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        }
        jobs_processed
    });

    let _ = producer_handle.await;
    let processed = consumer_handle.await.unwrap();

    let e2e_duration = start_e2e.elapsed();

    println!("{}: End-to-End Latency for {} jobs (Producer -> Queue -> Consumer) took {:?}", name, processed, e2e_duration);

    let latency_per_job = e2e_duration.as_micros() as f64 / processed as f64;
    println!("{}: Average e2e latency per job: {:.2} us", name, latency_per_job);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_ai_job_dispatch_latency() {
        bench_ai_job_dispatch_latency().await;
    }
}
