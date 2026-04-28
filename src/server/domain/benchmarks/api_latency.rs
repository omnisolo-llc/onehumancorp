use std::time::Instant;
use crate::domain::benchmarks::hybrid_latency::{BenchmarkResult, calculate_percentiles};
use crate::ohc::orchestration::{AgentManagerService, OrgService};
use crate::ohc::orchestration::{EmptyRequest};
use tonic::Request;
use crate::hub::Hub;
use std::sync::Arc;

pub async fn run_api_response_benchmark(mode: &str, pool: sqlx::PgPool) -> BenchmarkResult {
    println!("Running API Response Latency Benchmark in {} mode...", mode);

    // Wire real application services
    let (event_tx, _) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, pool.clone()));

    let agent_service = crate::services::agent::service::MyAgentManagerService::new(hub.clone());
    let org_service = crate::services::org::service::MyOrgService::new(hub.clone());

    let mut latencies = Vec::new();
    let samples = 50;

    for _ in 0..samples {
        let start = Instant::now();

        // Use parallel execution for real API endpoints instead of sleep
        let dashboard_future = org_service.get_analytics(Request::new(EmptyRequest {}));
        let agents_future = agent_service.get_agents(Request::new(EmptyRequest {}));
        let snapshots_future = agent_service.get_snapshots(Request::new(EmptyRequest {}));

        // Wait for all to complete in parallel (optimization)
        let _ = tokio::join!(dashboard_future, agents_future, snapshots_future);

        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }

    calculate_percentiles(latencies, "API Response Time (Parallel Execution)", mode)
}
