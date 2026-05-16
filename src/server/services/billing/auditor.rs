use std::collections::HashMap;
use std::sync::Mutex;
use ::server_pricing::calculator::{self, CostConfig};
use opentelemetry::global;
use opentelemetry::metrics::Counter;
use opentelemetry::KeyValue;

#[derive(Clone)]
pub struct AuditEvent {
    pub agent_id: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cached_input_tokens: i64,
    pub local_embedding_tokens: i64,
}

pub struct ComputeEvent {
    pub agent_id: String,
    pub compute_hours: f64,
    pub network_egress_bytes: i64,
}

pub struct CostAuditor {
    config: CostConfig,
    agent_costs: Mutex<HashMap<String, f64>>,
    agent_budgets: Mutex<HashMap<String, f64>>,
    total_cost: Mutex<f64>,
    caching_savings: Mutex<f64>,
    storage_savings: Mutex<f64>,
    total_compute_cost: Mutex<f64>,
    total_network_cost: Mutex<f64>,
    agent_revenues: Mutex<HashMap<String, f64>>,
    agent_output_tokens: Mutex<HashMap<String, i64>>,
    agent_storage_bytes: Mutex<HashMap<String, i64>>,
    telemetry_tx: Option<tokio::sync::mpsc::UnboundedSender<AuditEvent>>,
    llm_cost_counter: Counter<f64>,
    storage_savings_counter: Counter<f64>,
    compute_cost_counter: Counter<f64>,
}

impl CostAuditor {
    pub fn new(config: CostConfig) -> Self {
        let meter = global::meter("ohc.billing");
        let llm_cost_counter = meter.f64_counter("ohc_llm_cost_total").build();
        let storage_savings_counter = meter.f64_counter("ohc_storage_savings_total").build();
        let compute_cost_counter = meter.f64_counter("ohc_compute_cost_total").build();

        CostAuditor {
            config,
            agent_costs: Mutex::new(HashMap::new()),
            agent_budgets: Mutex::new(HashMap::new()),
            total_cost: Mutex::new(0.0),
            caching_savings: Mutex::new(0.0),
            storage_savings: Mutex::new(0.0),
            total_compute_cost: Mutex::new(0.0),
            total_network_cost: Mutex::new(0.0),
            agent_revenues: Mutex::new(HashMap::new()),
            agent_output_tokens: Mutex::new(HashMap::new()),
            agent_storage_bytes: Mutex::new(HashMap::new()),
            telemetry_tx: None,
            llm_cost_counter,
            storage_savings_counter,
            compute_cost_counter,
        }
    }

    pub fn set_telemetry_tx(&mut self, tx: tokio::sync::mpsc::UnboundedSender<AuditEvent>) {
        self.telemetry_tx = Some(tx);
    }

    pub fn record_event(&self, event: AuditEvent) -> f64 {
        let cost = calculator::calculate_cost_with_config(
            event.input_tokens,
            event.output_tokens,
            event.cached_input_tokens,
            event.local_embedding_tokens,
            &self.config,
        );

        let mut agent_costs = self.agent_costs.lock().unwrap();
        let mut total_cost = self.total_cost.lock().unwrap();

        let current_cost = agent_costs.entry(event.agent_id.clone()).or_insert(0.0);
        *current_cost += cost;
        *total_cost += cost;

        let mut agent_output_tokens = self.agent_output_tokens.lock().unwrap();
        let current_tokens = agent_output_tokens.entry(event.agent_id.clone()).or_insert(0);
        *current_tokens += event.output_tokens;

        self.llm_cost_counter.add(cost, &[KeyValue::new("agent_id", event.agent_id.clone())]);

        if let Some(tx) = &self.telemetry_tx {
            let _ = tx.send(event.clone());
        }

        cost
    }

    pub fn record_cache_hit(&self, event: AuditEvent) -> f64 {
        let actual_cost = calculator::calculate_cost_with_config(
            event.input_tokens,
            event.output_tokens,
            event.cached_input_tokens,
            event.local_embedding_tokens,
            &self.config,
        );
        let uncached_cost = calculator::calculate_cost_with_config(
            event.input_tokens + event.cached_input_tokens,
            event.output_tokens,
            0,
            event.local_embedding_tokens,
            &self.config,
        );
        let saved_cost = ((uncached_cost - actual_cost) * 10000.0).round() / 10000.0;

        let mut caching_savings = self.caching_savings.lock().unwrap();
        *caching_savings += saved_cost;

        saved_cost
    }

    pub fn get_agent_cost(&self, agent_id: &str) -> f64 {
        let agent_costs = self.agent_costs.lock().unwrap();
        *agent_costs.get(agent_id).unwrap_or(&0.0)
    }

    pub fn get_total_savings(&self) -> f64 {
        let caching_savings = self.caching_savings.lock().unwrap();
        *caching_savings
    }

    pub fn record_storage_compression(&self, original_bytes: i64, compressed_bytes: i64) -> f64 {
        let savings = calculator::calculate_storage_savings(original_bytes, compressed_bytes, &self.config);
        
        let mut storage_savings = self.storage_savings.lock().unwrap();
        *storage_savings += savings;
        
        self.storage_savings_counter.add(savings, &[]);

        savings
    }

    pub fn get_total_storage_savings(&self) -> f64 {
        let storage_savings = self.storage_savings.lock().unwrap();
        *storage_savings
    }

    pub fn get_total_cost(&self) -> f64 {
        let total_cost = self.total_cost.lock().unwrap();
        *total_cost
    }

    pub fn get_agent_costs_snapshot(&self) -> Vec<(String, f64, i64, f64, f64, i64)> {
        let agent_costs = self.agent_costs.lock().unwrap();
        let agent_revenues = self.agent_revenues.lock().unwrap();
        let agent_output_tokens = self.agent_output_tokens.lock().unwrap();
        let agent_storage_bytes = self.agent_storage_bytes.lock().unwrap();
        let mut result = Vec::new();
        for (agent_id, cost) in agent_costs.iter() {
            let revenue = agent_revenues.get(agent_id).unwrap_or(&0.0);
            let output_tokens = agent_output_tokens.get(agent_id).unwrap_or(&0);
            let storage_bytes = agent_storage_bytes.get(agent_id).unwrap_or(&0);
            let roi = self.calculate_roi(*cost, *revenue);
            let efficiency = self.calculate_efficiency(*cost, *output_tokens);
            result.push((agent_id.clone(), *cost, *output_tokens, roi, efficiency, *storage_bytes));
        }
        result
    }


    pub fn record_agent_storage(&self, agent_id: &str, bytes: i64) {
        let mut agent_storage_bytes = self.agent_storage_bytes.lock().unwrap();
        let current_bytes = agent_storage_bytes.entry(agent_id.to_string()).or_insert(0);
        *current_bytes += bytes;
    }

    pub fn get_total_tokens(&self) -> i64 {
        let agent_output_tokens = self.agent_output_tokens.lock().unwrap();
        agent_output_tokens.values().sum()
    }

    pub fn get_total_revenue(&self) -> f64 {
        let agent_revenues = self.agent_revenues.lock().unwrap();
        agent_revenues.values().sum()
    }

    pub fn calculate_roi(&self, cost: f64, revenue: f64) -> f64 {
        calculator::calculate_roi(cost, revenue)
    }

    pub fn calculate_efficiency(&self, cost: f64, output_tokens: i64) -> f64 {
        calculator::calculate_efficiency(cost, output_tokens)
    }

    pub fn record_revenue(&self, agent_id: &str, amount: f64) {
        let mut agent_revenues = self.agent_revenues.lock().unwrap();
        let current_revenue = agent_revenues.entry(agent_id.to_string()).or_insert(0.0);
        *current_revenue += amount;
    }

    pub fn record_compute_event(&self, event: ComputeEvent) -> f64 {
        let compute_cost = calculator::calculate_compute_cost(event.compute_hours, &self.config);
        let network_cost = calculator::calculate_network_cost(event.network_egress_bytes, &self.config);
        let total = compute_cost + network_cost;

        let mut agent_costs = self.agent_costs.lock().unwrap();
        let mut total_cost = self.total_cost.lock().unwrap();
        let mut total_compute_cost = self.total_compute_cost.lock().unwrap();
        let mut total_network_cost = self.total_network_cost.lock().unwrap();

        let current_cost = agent_costs.entry(event.agent_id.clone()).or_insert(0.0);
        *current_cost += total;
        *total_cost += total;
        *total_compute_cost += compute_cost;
        *total_network_cost += network_cost;

        self.compute_cost_counter.add(total, &[KeyValue::new("agent_id", event.agent_id.clone())]);

        total
    }

    pub fn generate_report(&self) -> String {
        let agent_costs = self.agent_costs.lock().unwrap();
        let agent_budgets = self.agent_budgets.lock().unwrap();
        let total_cost = self.total_cost.lock().unwrap();
        let caching_savings = self.caching_savings.lock().unwrap();
        let storage_savings = self.storage_savings.lock().unwrap();
        let total_compute_cost = self.total_compute_cost.lock().unwrap();
        let total_network_cost = self.total_network_cost.lock().unwrap();
        let agent_revenues = self.agent_revenues.lock().unwrap();
        let agent_output_tokens = self.agent_output_tokens.lock().unwrap();

        let mut report = format!("Total Cost: ${:.4}\n", *total_cost);
        report += &format!("Total Savings via Caching: ${:.4}\n", *caching_savings);
        report += &format!("Total Savings via Storage Compression: ${:.4}\n", *storage_savings);
        report += &format!("Total Compute Cost: ${:.4}\n", *total_compute_cost);
        report += &format!("Total Network Cost: ${:.4}\n", *total_network_cost);
        report += "Agent Costs:\n";

        for (agent_id, cost) in agent_costs.iter() {
            let revenue = agent_revenues.get(agent_id).unwrap_or(&0.0);
            let output_tokens = agent_output_tokens.get(agent_id).unwrap_or(&0);

            let roi = self.calculate_roi(*cost, *revenue);
            let efficiency = self.calculate_efficiency(*cost, *output_tokens);

            let metrics_str = format!(" [ROI: {:.2}%, Efficiency: {:.2} tok/$]", roi, efficiency);

            let budget = agent_budgets.get(agent_id);
            if let Some(budget) = budget {
                if cost > budget {
                    report += &format!("- {}: ${:.4} (OVER BUDGET){}\n", agent_id, cost, metrics_str);
                } else {
                    report += &format!("- {}: ${:.4}{}\n", agent_id, cost, metrics_str);
                }
            } else {
                report += &format!("- {}: ${:.4}{}\n", agent_id, cost, metrics_str);
            }
        }

        report
    }

    pub fn set_agent_budget(&self, agent_id: &str, budget: f64) {
        let mut agent_budgets = self.agent_budgets.lock().unwrap();
        agent_budgets.insert(agent_id.to_string(), budget);
    }

    pub fn is_agent_over_budget(&self, agent_id: &str) -> bool {
        let agent_costs = self.agent_costs.lock().unwrap();
        let agent_budgets = self.agent_budgets.lock().unwrap();

        let cost = agent_costs.get(agent_id).unwrap_or(&0.0);
        let budget = agent_budgets.get(agent_id);

        if let Some(budget) = budget {
            cost > budget
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_pricing::calculator::CostConfig;

    #[test]
    fn test_cost_auditor() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        };
        let auditor = CostAuditor::new(config);
        
        let event = AuditEvent {
            agent_id: "agent1".to_string(),
            input_tokens: 1000,
            output_tokens: 500,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        };
        

        let cost = auditor.record_event(event);
        assert_eq!(cost, 2.0); // 1000*0.001 + 500*0.002 = 1.0 + 1.0 = 2.0

        auditor.record_revenue("agent1", 5.0);

        assert_eq!(auditor.get_agent_cost("agent1"), 2.0);
        
        auditor.set_agent_budget("agent1", 1.0);
        assert!(auditor.is_agent_over_budget("agent1"));
        
        let report = auditor.generate_report();
        assert!(report.contains("OVER BUDGET"));
    }

    #[test]
    fn test_record_cache_hit() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0005,
            ..Default::default()
        };
        let auditor = CostAuditor::new(config);

        let event = AuditEvent {
            agent_id: "agent1".to_string(),
            input_tokens: 100,
            output_tokens: 50,
            cached_input_tokens: 100,
            local_embedding_tokens: 0,
        };

        let savings = auditor.record_cache_hit(event);
        assert!(savings > 0.0);
        assert_eq!(auditor.get_total_savings(), savings);
    }

    #[test]
    fn test_record_storage_compression() {
        let config = CostConfig {
            cost_per_gb_month: 0.1,
            ..Default::default()
        };
        let auditor = CostAuditor::new(config);

        let original_bytes = 1024 * 1024 * 1024 * 2; // 2GB
        let compressed_bytes = 1024 * 1024 * 1024 * 1; // 1GB

        let savings = auditor.record_storage_compression(original_bytes, compressed_bytes);
        assert_eq!(savings, 0.1);
        assert_eq!(auditor.get_total_storage_savings(), 0.1);
    }
}


#[derive(Debug, Clone)]
pub struct RegionRouting {
    pub region_id: String,
    pub base_cost_multiplier: f64,
    pub latency_ms: u32,
    pub is_available: bool,
}

pub struct RoutingOptimizer {
    regions: HashMap<String, RegionRouting>,
}

impl RoutingOptimizer {
    pub fn new() -> Self {
        let mut regions = HashMap::new();
        regions.insert("us-east".to_string(), RegionRouting { region_id: "us-east".to_string(), base_cost_multiplier: 1.0, latency_ms: 20, is_available: true });
        regions.insert("us-west".to_string(), RegionRouting { region_id: "us-west".to_string(), base_cost_multiplier: 1.1, latency_ms: 60, is_available: true });
        regions.insert("eu-central".to_string(), RegionRouting { region_id: "eu-central".to_string(), base_cost_multiplier: 1.2, latency_ms: 100, is_available: true });
        regions.insert("ap-south".to_string(), RegionRouting { region_id: "ap-south".to_string(), base_cost_multiplier: 0.9, latency_ms: 150, is_available: true });
        Self { regions }
    }

    pub fn get_optimal_region(&self, max_latency: u32, require_availability: bool) -> Option<String> {
        let mut best_region: Option<String> = None;
        let mut lowest_cost = f64::MAX;

        for (id, region) in &self.regions {
            if require_availability && !region.is_available {
                continue;
            }
            if region.latency_ms > max_latency {
                continue;
            }
            if region.base_cost_multiplier < lowest_cost {
                lowest_cost = region.base_cost_multiplier;
                best_region = Some(id.clone());
            }
        }
        best_region
    }

    pub fn update_region_status(&mut self, region_id: &str, is_available: bool, new_latency: u32) {
        if let Some(region) = self.regions.get_mut(region_id) {
            region.is_available = is_available;
            region.latency_ms = new_latency;
        }
    }
}

pub struct DiscountCalculator {
    pub base_rate: f64,
    pub volume_tiers: Vec<(u64, f64)>, // (threshold, discount_percentage)
}

impl DiscountCalculator {
    pub fn new(base_rate: f64) -> Self {
        Self {
            base_rate,
            volume_tiers: vec![
                (10_000, 0.05),   // 5% off over 10k units
                (50_000, 0.10),   // 10% off over 50k units
                (100_000, 0.20),  // 20% off over 100k units
                (1_000_000, 0.35),// 35% off over 1M units
            ],
        }
    }

    pub fn calculate_final_price(&self, volume: u64) -> f64 {
        let mut applicable_discount = 0.0;
        for (threshold, discount) in &self.volume_tiers {
            if volume >= *threshold {
                applicable_discount = *discount;
            }
        }

        let subtotal = (volume as f64) * self.base_rate;
        subtotal * (1.0 - applicable_discount)
    }

    pub fn calculate_savings(&self, volume: u64) -> f64 {
        let subtotal = (volume as f64) * self.base_rate;
        let final_price = self.calculate_final_price(volume);
        subtotal - final_price
    }
}

#[cfg(test)]
mod tests_additional {
    use super::*;

    #[test]
    fn test_routing_optimizer() {
        let mut optimizer = RoutingOptimizer::new();
        assert_eq!(optimizer.get_optimal_region(50, true), Some("us-east".to_string()));
        assert_eq!(optimizer.get_optimal_region(200, true), Some("ap-south".to_string()));

        optimizer.update_region_status("ap-south", false, 150);
        assert_eq!(optimizer.get_optimal_region(200, true), Some("us-east".to_string()));
    }

    #[test]
    fn test_discount_calculator() {
        let calc = DiscountCalculator::new(1.0);
        assert_eq!(calc.calculate_final_price(5000), 5000.0);
        assert_eq!(calc.calculate_final_price(10000), 9500.0); // 5% discount
        assert_eq!(calc.calculate_final_price(50000), 45000.0); // 10% discount
        assert_eq!(calc.calculate_savings(100000), 20000.0); // 20% discount savings
    }
}
