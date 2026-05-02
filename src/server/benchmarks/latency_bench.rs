use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

// Phase 1: Baseline / Phase 2: Parallel Fetching Optimization & Batching
pub async fn bench_latency() {
    println!("Benchmarking Latency Suite...");
    bench_queue_latency().await;
    bench_task_manager_latency().await;
    bench_hub_latency().await;
}

pub async fn bench_queue_latency() {
    println!("--- Benchmarking Queue Latency ---");

    // 1. Cloud Mode - Postgres
    println!("--- Cloud Mode (Postgres) ---");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url != "postgres://localhost/dummy" {
        let pool_res = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    let _ = conn.execute("RESET app.current_tenant").await;
                    Ok(true)
                })
            })
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SELECT set_config('app.current_tenant', 'system', false)").await?;
                    Ok(true)
                })
            })
            .connect(&database_url).await;

        if let Ok(pg_pool) = pool_res {
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

pub async fn bench_task_manager_latency() {
    println!("--- Benchmarking TaskManager Latency ---");
    let tm = Arc::new(crate::tasks::TaskManager::new());
    let iterations = 1000;
    let mut create_times = Vec::new();
    let mut claim_times = Vec::new();

    for i in 0..iterations {
        let start = Instant::now();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), format!("Task {}", i), "Desc".to_string(), "P1".to_string()).unwrap();
        create_times.push(start.elapsed().as_micros());

        let start_claim = Instant::now();
        let _ = tm.claim_task(&task.id, "agent1".to_string()).unwrap();
        claim_times.push(start_claim.elapsed().as_micros());
    }

    create_times.sort();
    claim_times.sort();
    println!("TaskManager Create: p50: {} us, p95: {} us, p99: {} us", create_times[iterations / 2], create_times[(iterations as f32 * 0.95) as usize], create_times[(iterations as f32 * 0.99) as usize]);
    println!("TaskManager Claim: p50: {} us, p95: {} us, p99: {} us", claim_times[iterations / 2], claim_times[(iterations as f32 * 0.95) as usize], claim_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_hub_latency() {
    println!("--- Benchmarking Hub Latency ---");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    // Use a dummy pool for in-memory hub benches
    let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
    let hub = Arc::new(crate::hub::Hub::new(tx, pool));
    let iterations = 1000;
    let mut register_times = Vec::new();
    let mut publish_times = Vec::new();

    for i in 0..iterations {
        let start = Instant::now();
        hub.register_agent(crate::ohc::orchestration::Agent {
            id: format!("agent-{}", i),
            name: format!("Agent {}", i),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
        register_times.push(start.elapsed().as_micros());

        let start_pub = Instant::now();
        let _ = hub.clone().publish(crate::ohc::orchestration::Message {
            id: format!("msg-{}", i),
            from_agent: format!("agent-{}", i),
            to_agent: "agent-0".to_string(),
            r#type: "text".to_string(),
            content: "hello".to_string(),
            meeting_id: String::new(),
            occurred_at_unix: Utc::now().timestamp(),
        });
        publish_times.push(start_pub.elapsed().as_micros());
    }

    register_times.sort();
    publish_times.sort();
    println!("Hub Register: p50: {} us, p95: {} us, p99: {} us", register_times[iterations / 2], register_times[(iterations as f32 * 0.95) as usize], register_times[(iterations as f32 * 0.99) as usize]);
    println!("Hub Publish: p50: {} us, p95: {} us, p99: {} us", publish_times[iterations / 2], publish_times[(iterations as f32 * 0.95) as usize], publish_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_dashboard_snapshot() {
    println!("Benchmarking Dashboard Snapshot Fetching...");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
    let hub = Arc::new(crate::hub::Hub::new(tx, pool));
    let service = crate::services::agent::service::MyAgentManagerService::new(hub.clone());

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

    // Warm up and first fetch (cache miss)
    let _ = service.get_snapshot().await;

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = service.get_snapshot().await;
        fetch_times.push(start.elapsed().as_micros());
    }

    fetch_times.sort();
    println!("Actual Service Snapshot Fetch (Cached): p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);
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
        bench_dashboard_snapshot().await;
    }

    #[tokio::test]
    async fn test_run_bench_latency_suite() {
        bench_latency().await;
    }
}
