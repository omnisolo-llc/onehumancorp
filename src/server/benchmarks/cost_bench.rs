use ::server_pricing::calculator::{calculate_cost, calculate_cost_cents, calculate_cost_with_config, CostConfig};
use std::time::Instant;

pub async fn bench_cost_calculator() {
    println!("Workload/cost instrumentation and repeatable benchmark");
    let start = Instant::now();
    for _i in 0..10000 {
        calculate_cost("gpt-4o", 1000000, 1000000, 0);
        calculate_cost_cents("claude-3-opus", 500000, 200000, 100000);
    }
    let duration = start.elapsed();
    println!("Performed 20000 cost calculations in {:?}", duration);

    let config = CostConfig {
        cost_per_input_token: 0.001,
        cost_per_output_token: 0.002,
        cost_per_cached_input_token: 0.0005,
        cost_per_local_embedding: 0.0001,
        discount_factor: 0.05,
        cost_per_gb_month: 0.10,
        cost_per_network_gb: 0.50,
        cost_per_compute_hour: 2.0,
        storage_quota_gb: 10,
    };

    let start2 = Instant::now();
    for _i in 0..10000 {
        calculate_cost_with_config(1000, 500, 200, 100, &config);
    }
    let duration2 = start2.elapsed();
    println!("Performed 10000 config-based cost calculations in {:?}", duration2);
}
