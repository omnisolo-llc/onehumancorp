use std::time::Instant;
use crate::db::DB;
use crate::queue::{SubAgentJob, PgSubAgentQueue, TaskQueue};
use sqlx::PgPool;
use chrono::Utc;

pub struct BenchmarkResult {
    pub name: String,
    pub mode: String,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub avg_ms: f64,
    pub samples: usize,
}

pub async fn run_hybrid_latency_benchmark(pool: PgPool, is_standalone: bool) -> Vec<BenchmarkResult> {
    let mode = if is_standalone { "Standalone".to_string() } else { "Cloud".to_string() };
    println!("Running Hybrid Latency Benchmark in {} mode...", mode);

    let mut results = Vec::new();

    // 1. Database Query Latency Benchmark
    results.push(benchmark_db_query(&pool, &mode).await);

    // 2. AI Job Dispatch Latency Benchmark
    results.push(benchmark_job_dispatch(pool.clone(), &mode).await);

    results
}

async fn benchmark_db_query(pool: &PgPool, mode: &str) -> BenchmarkResult {
    let mut latencies = Vec::new();
    let samples = 100;

    // Try to ensure table exists (assuming tasks table exists for this context)
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, status TEXT)").execute(pool).await;

    for _ in 0..samples {
        let start = Instant::now();
        // A simple query to measure baseline DB response
        let _ = sqlx::query("SELECT COUNT(*) FROM tasks").execute(pool).await;
        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }

    calculate_percentiles(latencies, "Database Query Latency", mode)
}

async fn benchmark_job_dispatch(pool: PgPool, mode: &str) -> BenchmarkResult {
    let queue = PgSubAgentQueue::new(pool.clone());
    let mut latencies = Vec::new();
    let samples = 50;

    // Try to ensure table exists
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT, parent_task_id TEXT, payload TEXT, status TEXT, scheduled_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)").execute(&pool).await;

    for i in 0..samples {
        let start = Instant::now();

        let job = SubAgentJob {
            id: format!("bench-job-{}", i),
            organization_id: "bench-org".to_string(),
            parent_task_id: "parent-1".to_string(),
            payload: serde_json::json!({"agent_role": "test"}),
            status: "PENDING".to_string(),
            scheduled_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let _ = queue.enqueue(job).await;

        let _ = queue.poll("test_worker").await;

        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }

    // cleanup
    let _ = sqlx::query("DELETE FROM sub_agent_queue WHERE organization_id = 'bench-org'").execute(&pool).await;

    calculate_percentiles(latencies, "AI Job Dispatch Latency", mode)
}

fn calculate_percentiles(mut latencies: Vec<f64>, name: &str, mode: &str) -> BenchmarkResult {
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let samples = latencies.len();
    if samples == 0 {
        return BenchmarkResult {
            name: name.to_string(),
            mode: mode.to_string(),
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            avg_ms: 0.0,
            samples: 0,
        }
    }
    let p50_idx = (samples as f64 * 0.50) as usize;
    let p95_idx = (samples as f64 * 0.95) as usize;
    let p99_idx = (samples as f64 * 0.99) as usize;

    let p50_ms = latencies.get(p50_idx).copied().unwrap_or(0.0);
    let p95_ms = latencies.get(p95_idx).copied().unwrap_or(0.0);
    let p99_ms = latencies.get(p99_idx).copied().unwrap_or(0.0);

    let avg_ms = latencies.iter().sum::<f64>() / samples as f64;

    BenchmarkResult {
        name: name.to_string(),
        mode: mode.to_string(),
        p50_ms,
        p95_ms,
        p99_ms,
        avg_ms,
        samples,
    }
}
