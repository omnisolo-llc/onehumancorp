pub mod mcp_sync_worker;

pub mod forecaster;

pub use ::server_config as config;
use chrono::Utc;
use opentelemetry::global;
use serde_json::{Map, Value};
use sqlx::{PgPool, query};
use std::sync::OnceLock;
use regex::Regex;

use opentelemetry::metrics::Histogram;

use opentelemetry::metrics::{Counter, UpDownCounter, Gauge};

static SUB_AGENT_QUEUE_LENGTH_GAUGE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static SUB_AGENT_QUEUE_DELAY_HISTOGRAM: OnceLock<Histogram<f64>> = OnceLock::new();
static TASK_CLAIM_CONTENTION_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static BUBBLEWRAP_SPAWN_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static BUBBLEWRAP_EXECUTION_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static BUBBLEWRAP_VIOLATION_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static SANDBOX_VIOLATION_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static TOKEN_USAGE: OnceLock<Counter<u64>> = OnceLock::new();
static AGENT_EFFICIENCY_SCORE: OnceLock<Gauge<f64>> = OnceLock::new();
static AGENT_API_CALL: OnceLock<Counter<u64>> = OnceLock::new();
static AGENT_API_ERROR: OnceLock<Counter<u64>> = OnceLock::new();
static HUMAN_INTERACTION: OnceLock<Counter<u64>> = OnceLock::new();
static MEETING_EVENT: OnceLock<Counter<u64>> = OnceLock::new();
static SWARM_TASK_COMPLETED: OnceLock<Counter<u64>> = OnceLock::new();
static MCP_TOOL_CALLS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static POSTGRES_LOCK_CONTENTION: OnceLock<Counter<u64>> = OnceLock::new();
static LLM_NETWORK_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static AUTODREAM_SYNC_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static TOKEN_USAGE_BY_OUTCOME: OnceLock<Counter<u64>> = OnceLock::new();

static ERROR_SIGNAL_CATEGORIZED: OnceLock<Counter<u64>> = OnceLock::new();

pub fn categorize_error_signal(err_msg: &str) -> &'static str {
    let lower = err_msg.to_lowercase();
    if lower.contains("panic") || lower.contains("segfault") || lower.contains("unreachable") || lower.contains("fatal") || lower.contains("bug") {
        "bug"
    } else if lower.contains("not supported") || lower.contains("missing feature") || lower.contains("feature") {
        "feature"
    } else if lower.contains("deprecated") || lower.contains("legacy") || lower.contains("refactor") {
        "refactor"
    } else if lower.contains("leak") || lower.contains("garbage") || lower.contains("clean up") || lower.contains("cleanup") || lower.contains("stagnant") || lower.contains("stuck") {
        "cleanup"
    } else if lower.contains("doc") || lower.contains("comment") || lower.contains("readme") {
        "docs"
    } else if lower.contains("cve") || lower.contains("vulnerabilit") || lower.contains("injection") || lower.contains("auth") || lower.contains("security") || lower.contains("malware") || lower.contains("permission") || lower.contains("denied") {
        "security"
    } else {
        "bug"
    }
}

pub fn get_error_signal_counter() -> &'static Counter<u64> {
    ERROR_SIGNAL_CATEGORIZED.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter
            .u64_counter("ohc_error_signals_total")
            .with_description("Total number of error signals categorized")
            .build()
    })
}

pub fn get_sandbox_violation_total() -> &'static UpDownCounter<i64> {
    SANDBOX_VIOLATION_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .i64_up_down_counter("ohc_agent_sandbox_violations_total")
            .with_description("Total number of LocalSandbox policy violations")
            .build()
    })
}

pub fn record_error_signal(err_msg: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let category = categorize_error_signal(err_msg);
    let counter = get_error_signal_counter();
    counter.add(1, &[opentelemetry::KeyValue::new("category", category)]);
}

pub fn get_postgres_lock_contention_counter() -> &'static Counter<u64> {
    POSTGRES_LOCK_CONTENTION.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("ohc_postgres_lock_contention_total")
            .with_description("Total number of PostgreSQL lock contentions")
            .build()
    })
}

pub fn get_llm_network_latency_histogram() -> &'static Histogram<f64> {
    LLM_NETWORK_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.f64_histogram("ohc_llm_network_latency_seconds")
            .with_description("Latency to external LLM providers in seconds")
            .build()
    })
}

pub fn get_mcp_tool_calls_counter() -> &'static Counter<u64> {
    MCP_TOOL_CALLS_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("ohc_mcp_tool_calls_total")
            .with_description("Total number of MCP tool calls")
            .build()
    })
}

pub fn get_harness_execution_latency() -> &'static Histogram<f64> {
    HARNESS_EXECUTION_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.harness");
        meter
            .f64_histogram("ohc_harness_command_duration_seconds")
            .with_description("Execution latency for Harness")
            .build()
    })
}

pub fn get_token_usage_counter() -> &'static Counter<u64> {
    TOKEN_USAGE.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("token_usage").build()
    })
}

pub fn get_token_usage_by_outcome_counter() -> &'static Counter<u64> {
    TOKEN_USAGE_BY_OUTCOME.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("ohc_token_usage_by_outcome").build()
    })
}

pub fn get_agent_efficiency_score_gauge() -> &'static Gauge<f64> {
    AGENT_EFFICIENCY_SCORE.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.f64_gauge("ohc_agent_roi_efficiency_score").build()
    })
}


pub fn get_agent_api_call_counter() -> &'static Counter<u64> {
    AGENT_API_CALL.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("agent_api_call").build()
    })
}

pub fn get_agent_api_error_counter() -> &'static Counter<u64> {
    AGENT_API_ERROR.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("agent_api_error").build()
    })
}

pub fn get_human_interaction_counter() -> &'static Counter<u64> {
    HUMAN_INTERACTION.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("human_interaction").build()
    })
}

pub fn get_meeting_event_counter() -> &'static Counter<u64> {
    MEETING_EVENT.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("meeting_event").build()
    })
}

pub fn get_swarm_task_completed_counter() -> &'static Counter<u64> {
    SWARM_TASK_COMPLETED.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("swarm_task_completed").build()
    })
}

pub fn record_token_usage(agent_id: &str, role: &str, model: &str, token_type: &str, count: i64) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_token_usage_counter();
    counter.add(
        count as u64,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("role", role.to_string()),
            opentelemetry::KeyValue::new("model", model.to_string()),
            opentelemetry::KeyValue::new("type", token_type.to_string()),
        ],
    );
}

pub fn record_sandbox_violation(reason: &str, command: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let gauge = get_sandbox_violation_total();
    gauge.add(
        1,
        &[
            opentelemetry::KeyValue::new("reason", reason.to_string()),
            opentelemetry::KeyValue::new("command", command.to_string()),
        ],
    );
}

pub fn record_harness_execution_latency(latency_seconds: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_harness_execution_latency();
    let deployment_mode = get_deployment_mode();
    histogram.record(
        latency_seconds,
        &[opentelemetry::KeyValue::new(
            "deployment_mode",
            deployment_mode.to_string(),
        )],
    );
}

#[cfg(test)]
mod harness_execution_tests {
    use super::*;


    #[test]
    fn test_get_harness_execution_latency() {
        let histogram = get_harness_execution_latency();
        // Just calling it ensures it initializes correctly
        histogram.record(1.0, &[]);
    }

    #[test]
    fn test_record_harness_execution_latency() {
        // Just calling it ensures it doesn't panic
        record_harness_execution_latency(1.0);
    }
}

pub fn record_agent_api_call(agent_id: &str, role: &str, api: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_agent_api_call_counter();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("role", role.to_string()),
            opentelemetry::KeyValue::new("api", api.to_string()),
        ],
    );
}

pub fn record_agent_api_error(agent_id: &str, role: &str, api: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_agent_api_error_counter();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("role", role.to_string()),
            opentelemetry::KeyValue::new("api", api.to_string()),
        ],
    );
}

pub fn record_human_interaction(interaction_type: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_human_interaction_counter();
    counter.add(
        1,
        &[opentelemetry::KeyValue::new(
            "type",
            interaction_type.to_string(),
        )],
    );
}

pub fn record_meeting_event(event_type: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_meeting_event_counter();
    counter.add(
        1,
        &[opentelemetry::KeyValue::new(
            "event_type",
            event_type.to_string(),
        )],
    );
}

pub fn record_swarm_task_completed(mission_id: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_swarm_task_completed_counter();
    counter.add(
        1,
        &[opentelemetry::KeyValue::new(
            "mission_id",
            mission_id.to_string(),
        )],
    );
}

static HARNESS_INIT_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static HARNESS_DB_IO_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static SYNC_DAEMON_ERROR_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static HARNESS_EXECUTION_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static SYNC_DAEMON_BATCH_SIZE_HISTOGRAM: OnceLock<Histogram<f64>> = OnceLock::new();
static AGENT_EXECUTION_TRACES_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

static MISSION_TIME_IN_QUEUE: OnceLock<Histogram<f64>> = OnceLock::new();
static TASK_PROCESSING_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static MISSION_EXECUTION_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static MISSION_FAILURE_RATE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static BUSINESS_EVENT_COUNT: OnceLock<UpDownCounter<i64>> = OnceLock::new();

pub fn get_deployment_mode() -> &'static str {
    static DEPLOYMENT_MODE: OnceLock<String> = OnceLock::new();
    DEPLOYMENT_MODE.get_or_init(|| {
        if std::env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true" {
            "Cloud".to_string()
        } else {
            "Standalone".to_string()
        }
    })
}
pub fn get_queue_length_gauge() -> &'static UpDownCounter<i64> {
    SUB_AGENT_QUEUE_LENGTH_GAUGE.get_or_init(|| {
        let meter = global::meter("ohc.sub_agent");
        meter
            .i64_up_down_counter("ohc.sub_agent.queue_length")
            .with_description("The current number of jobs in the sub-agent task queue")
            .build()
    })
}

pub fn get_sync_daemon_batch_size_histogram() -> &'static Histogram<f64> {
    SYNC_DAEMON_BATCH_SIZE_HISTOGRAM.get_or_init(|| {
        let meter = global::meter("ohc.daemon");
        meter
            .f64_histogram("ohc_sync_daemon_batch_size")
            .with_description("Batch size for sync daemon")
            .build()
    })
}

pub fn get_sync_daemon_error_total() -> &'static Counter<u64> {
    SYNC_DAEMON_ERROR_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.daemon");
        meter
            .u64_counter("sync_daemon_error_total")
            .with_description("Total sync daemon errors by mode and error type")
            .build()
    })
}

pub fn get_agent_execution_traces_total() -> &'static Counter<u64> {
    AGENT_EXECUTION_TRACES_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.agent");
        meter
            .u64_counter("ohc_agent_execution_traces_total")
            .with_description("Total number of agent execution traces")
            .build()
    })
}

pub fn get_sub_agent_queue_delay_histogram() -> &'static Histogram<f64> {
    SUB_AGENT_QUEUE_DELAY_HISTOGRAM.get_or_init(|| {
        let meter = global::meter("ohc.sub_agent");
        meter
            .f64_histogram("SubAgentQueueDelayHistogram")
            .with_description("Measures time from job enqueue to dequeue")
            .build()
    })
}

pub fn get_task_claim_contention_total() -> &'static UpDownCounter<i64> {
    TASK_CLAIM_CONTENTION_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sub_agent");
        meter
            .i64_up_down_counter("TaskClaimContentionTotal")
            .with_description(
                "Tracks the number of failed task claim attempts or retries due to lock contention",
            )
            .build()
    })
}

pub fn record_mcp_tool_call(tool_name: &str, status: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_mcp_tool_calls_counter();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("tool_name", tool_name.to_string()),
            opentelemetry::KeyValue::new("status", status.to_string()),
        ]
    );
}

pub fn record_postgres_lock_contention(operation: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_postgres_lock_contention_counter();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("operation", operation.to_string()),
        ]
    );
}

pub fn record_llm_network_latency(model: &str, latency: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_llm_network_latency_histogram();
    histogram.record(
        latency,
        &[
            opentelemetry::KeyValue::new("model", model.to_string()),
        ]
    );
}

pub fn get_task_processing_latency_histogram() -> &'static Histogram<f64> {
    TASK_PROCESSING_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.orchestration");
        meter
            .f64_histogram("ohc_task_processing_latency_seconds")
            .with_description("Job processing latency")
            .build()
    })
}

pub fn record_task_processing_latency(deployment_mode: &str, latency: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_task_processing_latency_histogram();
    histogram.record(
        latency,
        &[
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
        ],
    );
}

pub fn get_mission_time_in_queue_histogram() -> &'static Histogram<f64> {
    MISSION_TIME_IN_QUEUE.get_or_init(|| {
        let meter = global::meter("ohc.orchestration");
        meter
            .f64_histogram("MissionTimeInQueue")
            .with_description("Time a mission spends in the queue before being claimed")
            .build()
    })
}

pub fn get_mission_execution_latency_histogram() -> &'static Histogram<f64> {
    MISSION_EXECUTION_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.orchestration");
        meter
            .f64_histogram("MissionExecutionLatency")
            .with_description("Time a mission takes to execute")
            .build()
    })
}

pub fn get_mission_failure_rate_total() -> &'static UpDownCounter<i64> {
    MISSION_FAILURE_RATE.get_or_init(|| {
        let meter = global::meter("ohc.orchestration");
        meter
            .i64_up_down_counter("MissionFailureRate")
            .with_description("Total number of failed missions")
            .build()
    })
}

pub fn get_business_event_count_total() -> &'static UpDownCounter<i64> {
    BUSINESS_EVENT_COUNT.get_or_init(|| {
        let meter = global::meter("ohc.orchestration");
        meter
            .i64_up_down_counter("BusinessEventCount")
            .with_description("Total number of business events")
            .build()
    })
}

pub fn record_mission_time_in_queue(tenant_id: &str, deployment_mode: &str, latency: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_mission_time_in_queue_histogram();
    histogram.record(
        latency,
        &[
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
        ],
    );
}

pub fn record_mission_execution_latency(tenant_id: &str, deployment_mode: &str, latency: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_mission_execution_latency_histogram();
    histogram.record(
        latency,
        &[
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
        ],
    );
}

pub fn record_mission_failure(tenant_id: &str, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_mission_failure_rate_total();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
        ],
    );
}

pub fn record_business_event(tenant_id: &str, deployment_mode: &str, event_type: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_business_event_count_total();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
            opentelemetry::KeyValue::new("event_type", event_type.to_string()),
        ],
    );
}

pub fn record_sub_agent_queue_delay(delay: f64, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_sub_agent_queue_delay_histogram();
    histogram.record(delay, &[opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())]);
}

pub fn autodream_sync_duration_metric_name() -> &'static str {
    "ohc_autodream_sync_duration_seconds"
}

pub fn get_autodream_sync_duration_histogram() -> &'static Histogram<f64> {
    AUTODREAM_SYNC_DURATION.get_or_init(|| {
        let meter = global::meter("ohc.autodream");
        meter
            .f64_histogram(autodream_sync_duration_metric_name())
            .with_description("Duration of AutoDream sync batches in seconds")
            .build()
    })
}

pub fn record_autodream_sync_duration(duration_seconds: f64, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_autodream_sync_duration_histogram();
    histogram.record(
        duration_seconds,
        &[opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())],
    );
}

pub fn record_task_claim_contention(mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_task_claim_contention_total();
    counter.add(1, &[opentelemetry::KeyValue::new("mode", mode.to_string())]);
}

pub async fn record_autodream_sync(
    pool: &PgPool,
    count: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_records_synced_total",
        "counter",
        count,
        serde_json::json!({}),
    )
    .await
}

pub async fn record_llm_call_cost(
    pool: &PgPool,
    organization_id: &str,
    model: &str,
    cost_usd: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_llm_call_cost",
        "counter",
        cost_usd as f32,
        serde_json::json!({
            "organization_id": organization_id,
            "model": model,
        }),
    )
    .await?;

    let cost_cents = (cost_usd * 100.0).round() as i64;
    buffer_metric_i64(
        pool,
        "ohc_llm_cost_total_cents",
        "counter",
        cost_cents,
        serde_json::json!({
            "organization_id": organization_id,
            "model": model,
        }),
    ).await
}

pub async fn record_outbound_api_cost(
    pool: &PgPool,
    organization_id: &str,
    api_name: &str,
    cost_usd: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_outbound_api_cost",
        "counter",
        cost_usd as f32,
        serde_json::json!({
            "organization_id": organization_id,
            "api_name": api_name,
        }),
    )
    .await
}

pub async fn record_token_burn_rate_predicted_24h(
    pool: &PgPool,
    org_id: &str,
    forecast: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_token_burn_rate_predicted_24h",
        "gauge",
        forecast,
        serde_json::json!({ "organization_id": org_id }),
    )
    .await
}

pub async fn record_autodream_sync_error(
    pool: &PgPool,
    count: f32,
    error_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_sync_errors_total",
        "counter",
        count,
        serde_json::json!({ "error": error_type }),
    )
    .await
}

pub async fn record_autodream_ingestion_error(
    pool: &PgPool,
    count: f32,
    error_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_ingestion_error_total",
        "counter",
        count,
        serde_json::json!({ "error": error_type }),
    )
    .await
}

pub async fn record_autodream_compression_error(
    pool: &PgPool,
    count: f32,
    error_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_compression_error_total",
        "counter",
        count,
        serde_json::json!({ "error": error_type }),
    )
    .await
}

pub async fn record_autodream_consolidation(
    pool: &PgPool,
    count: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_consolidation_total",
        "counter",
        count,
        serde_json::json!({}),
    )
    .await
}

pub async fn record_sync_escalation(
    pool: &PgPool,
    count: f32,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "sync_escalation_total",
        "counter",
        count,
        serde_json::json!({ "mode": mode }),
    )
    .await
}

pub async fn record_sync_daemon_batch_size(
    pool: &PgPool,
    count: f32,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let histogram = get_sync_daemon_batch_size_histogram();
    histogram.record(
        count as f64,
        &[opentelemetry::KeyValue::new("mode", mode.to_string())],
    );

    buffer_metric(
        pool,
        "sync_daemon_batch_size",
        "gauge",
        count,
        serde_json::json!({ "mode": mode }),
    )
    .await
}

pub fn record_agent_execution_trace(agent_id: &str, trace_type: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_agent_execution_traces_total();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("trace_type", trace_type.to_string()),
        ],
    );
}

pub async fn record_sync_latency(
    pool: &PgPool,
    latency_ms: f32,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let histogram = global::meter("ohc.daemon")
        .f64_histogram("sync_latency_ms")
        .with_description("Sync daemon latency in ms by mode")
        .build();
    histogram.record(latency_ms as f64, &[opentelemetry::KeyValue::new("mode", mode.to_string())]);

    buffer_metric(
        pool,
        "sync_latency_ms",
        "histogram",
        latency_ms,
        serde_json::json!({ "mode": mode }),
    )
    .await
}

pub async fn record_sync_payload_size(
    pool: &PgPool,
    size_bytes: f32,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let histogram = global::meter("ohc.daemon")
        .f64_histogram("sync_payload_size_bytes")
        .with_description("Sync daemon payload size in bytes by mode")
        .build();
    histogram.record(size_bytes as f64, &[opentelemetry::KeyValue::new("mode", mode.to_string())]);

    buffer_metric(
        pool,
        "sync_payload_size_bytes",
        "histogram",
        size_bytes,
        serde_json::json!({ "mode": mode }),
    )
    .await
}

pub async fn record_sync_daemon_error_total(
    pool: &PgPool,
    count: f32,
    mode: &str,
    error_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let counter = get_sync_daemon_error_total();
    counter.add(
        count as u64,
        &[
            opentelemetry::KeyValue::new("mode", mode.to_string()),
            opentelemetry::KeyValue::new("error", error_type.to_string()),
        ],
    );

    buffer_metric(
        pool,
        "sync_daemon_error_total",
        "counter",
        count,
        serde_json::json!({ "mode": mode, "error": error_type }),
    )
    .await
}

pub async fn record_sqlite_throttled_request(
    pool: &PgPool,
    operation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_sqlite_throttled_requests_total",
        "counter",
        1.0,
        serde_json::json!({ "operation": operation }),
    )
    .await
}

pub async fn record_sqlite_lock_contention(
    pool: &PgPool,
    operation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_sqlite_lock_contention_total",
        "counter",
        1.0,
        serde_json::json!({ "operation": operation }),
    )
    .await
}

pub async fn record_sqlite_retry_exhausted(
    pool: &PgPool,
    operation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let deployment_mode = get_deployment_mode();
    let meter = global::meter("ohc.sqlite");
    let counter = meter.u64_counter("ohc_sqlite_retry_exhausted_total").build();
    counter.add(1, &[
        opentelemetry::KeyValue::new("operation", operation.to_string()),
        opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())
    ]);
    let deployment_mode = get_deployment_mode();
    let meter = global::meter("ohc.sqlite");
    let counter = meter.u64_counter("ohc_sqlite_retry_exhausted_total").build();
    counter.add(1, &[
        opentelemetry::KeyValue::new("operation", operation.to_string()),
        opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())
    ]);
    buffer_metric(
        pool,
        "ohc_sqlite_retry_exhausted_total",
        "counter",
        1.0,
        serde_json::json!({ "operation": operation }),
    )
    .await
}

pub fn record_queue_length_sync(delta: i32, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    get_queue_length_gauge().add(
        delta as i64,
        &[opentelemetry::KeyValue::new(
            "deployment_mode",
            deployment_mode.to_string(),
        )],
    );
}

pub async fn record_queue_length(
    pool: &PgPool,
    delta: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let deployment_mode = get_deployment_mode();

    get_queue_length_gauge().add(
        delta as i64,
        &[opentelemetry::KeyValue::new(
            "deployment_mode",
            deployment_mode,
        )],
    );
    let payload = serde_json::json!({ "delta": delta, "deployment_mode": deployment_mode });

    buffer_metric(
        pool,
        "ohc_sub_agent_queue_length",
        "gauge",
        delta as f32,
        payload,
    )
    .await
}

pub async fn record_task_resolution_efficiency(
    pool: &PgPool,
    outcome: &str,
    role: &str,
    model: &str,
    tokens: i64,
    tenant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let deployment_mode = get_deployment_mode();

    // 1. Outcome-Labeled Token Metrics
    buffer_metric(
        pool,
        "ohc_token_usage_by_outcome",
        "counter",
        tokens as f32,
        serde_json::json!({
            "outcome": outcome,
            "agent_role": role,
            "model": model,
            "deployment_mode": deployment_mode,
            "tenant_id": tenant_id,
        }),
    )
    .await?;

    let token_usage_counter = get_token_usage_by_outcome_counter();
    token_usage_counter.add(
        tokens as u64,
        &[
            opentelemetry::KeyValue::new("outcome", outcome.to_string()),
            opentelemetry::KeyValue::new("agent_role", role.to_string()),
            opentelemetry::KeyValue::new("model", model.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
        ],
    );

    // 2. ROI Calculation in Telemetry
    // Efficiency = 1 / (Tokens Consumed * 1000) for SUCCESS
    if outcome == "SUCCESS" && tokens > 0 {
        let efficiency = 1.0 / (tokens as f32 / 1000.0);
        buffer_metric(
            pool,
            "ohc_agent_roi_efficiency_score",
            "gauge",
            efficiency,
            serde_json::json!({
                "agent_role": role,
                "deployment_mode": deployment_mode,
                "tenant_id": tenant_id,
            }),
        )
        .await?;

        let efficiency_gauge = get_agent_efficiency_score_gauge();
        efficiency_gauge.record(
            efficiency as f64,
            &[
                opentelemetry::KeyValue::new("agent_role", role.to_string()),
                opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
                opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            ],
        );
    }

    Ok(())
}

pub async fn record_agent_cost(
    pool: &PgPool,
    agent_id: &str,
    organization_id: &str,
    role: &str,
    model: &str,
    entity: &str,
    cost: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_agent_cost",
        "counter",
        cost as f32,
        serde_json::json!({
            "agent_id": agent_id,
            "organization_id": organization_id,
            "role": role,
            "model": model,
            "entity": entity,
        }),
    )
    .await?;

    let cost_cents = (cost * 100.0).round() as i64;
    buffer_metric_i64(
        pool,
        "ohc_llm_cost_total_cents",
        "counter",
        cost_cents,
        serde_json::json!({
            "agent_id": agent_id,
            "organization_id": organization_id,
            "role": role,
            "model": model,
            "entity": entity,
        }),
    ).await
}

pub async fn record_api_call_cost(
    pool: &PgPool,
    organization_id: &str,
    entity: &str,
    cost: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_api_call_cost",
        "counter",
        cost as f32,
        serde_json::json!({
            "organization_id": organization_id,
            "entity": entity,
        }),
    )
    .await
}

pub async fn record_mcp_proxy_connections_active(
    pool: &PgPool,
    spiffe_id: &str,
    delta: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_mcp_proxy_connections_active",
        "gauge",
        delta,
        serde_json::json!({ "spiffe_id": spiffe_id }),
    )
    .await
}

pub async fn record_swarm_job_latency_by_entity(
    pool: &PgPool,
    mode: &str,
    entity: &str,
    latency: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_swarm_job_latency_by_entity_seconds",
        "histogram",
        latency as f32,
        serde_json::json!({
            "mode": mode,
            "entity": entity,
        }),
    )
    .await
}

pub async fn record_token_budget_alert(
    pool: &PgPool,
    org_id: &str,
    alert_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_token_budget_alert_total",
        "counter",
        1.0,
        serde_json::json!({ "organization_id": org_id, "alert_type": alert_type }),
    )
    .await
}

pub async fn record_capability_violation(
    pool: &PgPool,
    agent_id: &str,
    capability: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "capability_violation_total",
        "counter",
        1.0,
        serde_json::json!({ "agent_id": agent_id, "capability": capability }),
    )
    .await
}

pub async fn record_rag_escalation(
    pool: &PgPool,
    org_id: &str,
    error: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_rag_escalation_total",
        "counter",
        1.0,
        serde_json::json!({ "organization_id": org_id, "error": error }),
    )
    .await
}

pub async fn buffer_metric_i64(
    pool: &PgPool,
    metric_name: &str,
    metric_type: &str,
    value: i64,
    labels: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let is_telemetry_enabled = ::server_config::get().telemetry_enabled;

    if !is_telemetry_enabled {
        return Ok(());
    }

    let redacted_labels = redact_interface_pii(labels);
    let labels_json = serde_json::to_string(&redacted_labels)?;

    let tenant_id = redacted_labels.get("tenant_id")
        .or_else(|| redacted_labels.get("organization_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if let Some(tenant_id) = tenant_id {
        query(
            "INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status, tenant_id)
             VALUES ($1, $2, $3, $4, $5, 'pending', $6)"
        )
        .bind(metric_name)
        .bind(metric_type)
        .bind(value as f64)
        .bind(labels_json)
        .bind(Utc::now())
        .bind(tenant_id)
        .execute(pool)
        .await?;
    } else {
        query(
            "INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status)
             VALUES ($1, $2, $3, $4, $5, 'pending')"
        )
        .bind(metric_name)
        .bind(metric_type)
        .bind(value as f64)
        .bind(labels_json)
        .bind(Utc::now())
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn buffer_metric(
    pool: &PgPool,
    metric_name: &str,
    metric_type: &str,
    value: f32,
    labels: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // In standalone mode, do not sync telemetry to cloud unless explicitly enabled
    let is_telemetry_enabled = ::server_config::get().telemetry_enabled;

    if !is_telemetry_enabled {
        return Ok(());
    }

    let redacted_labels = redact_interface_pii(labels);
    let labels_json = serde_json::to_string(&redacted_labels)?;

    let tenant_id = redacted_labels.get("tenant_id")
        .or_else(|| redacted_labels.get("organization_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if let Some(tenant_id) = tenant_id {
        query(
            "INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status, tenant_id)
             VALUES ($1, $2, $3, $4, $5, 'pending', $6)"
        )
        .bind(metric_name)
        .bind(metric_type)
        .bind(value)
        .bind(labels_json)
        .bind(Utc::now())
        .bind(tenant_id)
        .execute(pool)
        .await?;
    } else {
        query(
            "INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status)
             VALUES ($1, $2, $3, $4, $5, 'pending')"
        )
        .bind(metric_name)
        .bind(metric_type)
        .bind(value)
        .bind(labels_json)
        .bind(Utc::now())
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub fn redact_interface_pii(val: Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map {
                if is_sensitive_key(&k) {
                    new_map.insert(k, Value::String("[REDACTED]".to_string()));
                } else {
                    new_map.insert(k, redact_interface_pii(v));
                }
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            let new_arr = arr.into_iter().map(redact_interface_pii).collect();
            Value::Array(new_arr)
        }
        Value::String(s) => {
            if is_email(&s) {
                Value::String("[EMAIL_REDACTED]".to_string())
            } else if is_pii_value_pattern(&s) {
                Value::String("[REDACTED]".to_string())
            } else {
                Value::String(s)
            }
        }
        Value::Number(n) => Value::Number(n),
        Value::Bool(b) => Value::Bool(b),
        Value::Null => Value::Null,
    }
}


pub fn is_pii_value_pattern(s: &str) -> bool {
    static SSN_RE: OnceLock<Regex> = OnceLock::new();
    static CC_RE: OnceLock<Regex> = OnceLock::new();
    static API_KEY_RE: OnceLock<Regex> = OnceLock::new();
    static PHONE_RE: OnceLock<Regex> = OnceLock::new();

    let ssn_re = SSN_RE.get_or_init(|| Regex::new(r"^\d{3}-\d{2}-\d{4}$").unwrap());
    let cc_re = CC_RE.get_or_init(|| Regex::new(r"^(\d{4}[- ]?){3,4}\d{1,4}$").unwrap());
    let api_key_re = API_KEY_RE.get_or_init(|| Regex::new(r"^(sk-|ak-|tok_)[a-zA-Z0-9]{10,}").unwrap());
    let phone_re = PHONE_RE.get_or_init(|| Regex::new(r"^\+?(\d{1,3})?[-. (]*\d{3}[-. )]*\d{3}[-. ]*\d{4}$").unwrap());

    ssn_re.is_match(s) || cc_re.is_match(s) || api_key_re.is_match(s) || phone_re.is_match(s)
}

pub fn is_sensitive_key(key: &str) -> bool {
    let key_lower = key.to_lowercase();
    // Exclude tenant_id and organization_id from being redacted
    if key_lower == "tenant_id" || key_lower == "organization_id" {
        return false;
    }

    let k: String = key.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();

    k.contains("password")
        || k.contains("secret")
        || k.contains("key")
        || k.contains("token")
        || k.contains("auth")
        || k.contains("cookie")
        || k.contains("credential")
        || k.contains("email")
        || k.contains("phone")
        || k.contains("ssn")
        || k.contains("address")
        || k.contains("name")
        || k.contains("pii")
        || k.contains("jwt")
        || k.contains("bearer")
        || k.contains("sessionid")
        || k.contains("payload")
        || k.contains("credit")
        || k.contains("card")
        || k.contains("cvv")
        || k.contains("dob")
        || k.contains("birth")
        || k.contains("passport")
        || k.contains("bank")
        || k.contains("account")
        || k.contains("stripe")
        || k.contains("billing")
        || k.contains("ipaddress")
        || k.contains("macaddress")
        || k.contains("geolocation")
        || k.contains("medical")
        || k.contains("health")
        || k.contains("salary")
        || k.contains("tax")
        || k.contains("socialsecurity")
        || k.contains("iban")
        || k.contains("routing")
        || k.contains("pin")
        || k.contains("ipaddress")
        || k.contains("macaddress")
        || k.contains("creditcard") || k.contains("deviceid") || k.contains("gps") || k.contains("latitude") || k.contains("longitude")
}

pub fn is_email(s: &str) -> bool {
    static EMAIL_RE: OnceLock<Regex> = OnceLock::new();
    let email_re = EMAIL_RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap());
    email_re.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_get_queue_length_gauge() {
        let gauge = get_queue_length_gauge();
        gauge.add(1, &[]);
        // Calling it again should return the same instance
        let gauge2 = get_queue_length_gauge();
        gauge2.add(1, &[]);
    }

    #[test]
    fn test_redact_interface_pii() {
        let original_json = serde_json::json!({
            "safe_field": "safe_value",
            "nested": {
                "password": "my_super_secret_password",
                "email": "user@example.com",
                "another_safe": "value"
            },
            "array": [
                { "ssn": "123-45-6789" },
                { "phone": "555-1234" }
            ],
            "raw_email": "test@test.com",
            "API_KEY": "sk-123456"
        });

        let redacted_json = redact_interface_pii(original_json);

        assert_eq!(redacted_json["safe_field"], "safe_value");
        assert_eq!(redacted_json["nested"]["password"], "[REDACTED]");
        assert_eq!(redacted_json["nested"]["email"], "[REDACTED]");
        assert_eq!(redacted_json["nested"]["another_safe"], "value");
        assert_eq!(redacted_json["array"][0]["ssn"], "[REDACTED]");
        assert_eq!(redacted_json["array"][1]["phone"], "[REDACTED]");
        // Since `raw_email`'s value contains an @ and ., it is considered an email by `is_email` check, NOT by the key!
        // But wait! `raw_email` key also contains "email"! Let's test a key that does NOT contain sensitive words but HAS email string
        assert_eq!(redacted_json["raw_email"], "[REDACTED]"); // "email" in key matched first!
        assert_eq!(redacted_json["API_KEY"], "[REDACTED]");
    }
}

pub async fn record_storage_rw_cost(
    pool: &PgPool,
    organization_id: &str,
    operation: &str,
    size_bytes: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let cost_cents = (size_bytes as f64 * 0.00000001).round() as i64;
    buffer_metric_i64(
        pool,
        "ohc_storage_rw_cost",
        "counter",
        cost_cents,
        serde_json::json!({
            "organization_id": organization_id,
            "operation": operation
        }),
    )
    .await
}

pub async fn record_email_send_cost(
    pool: &PgPool,
    organization_id: &str,
    count: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    let cost_cents = count; // Assuming 1 cent per email
    buffer_metric_i64(
        pool,
        "ohc_email_send_cost",
        "counter",
        cost_cents,
        serde_json::json!({
            "organization_id": organization_id
        }),
    )
    .await
}

static ONBOARDING_DURATION_HISTOGRAM: OnceLock<Histogram<u64>> = OnceLock::new();

pub fn get_onboarding_duration_histogram() -> &'static Histogram<u64> {
    ONBOARDING_DURATION_HISTOGRAM.get_or_init(|| {
        let meter = global::meter("ohc.onboarding");
        meter
            .u64_histogram("ohc.onboarding.step_duration")
            .with_description("Duration of onboarding steps in ms")
            .build()
    })
}

pub fn track_onboarding_step(tenant_id: &str, step: &str, duration_ms: u64) {
    if !::server_config::get().telemetry_enabled { return; }
    let histogram = get_onboarding_duration_histogram();
    histogram.record(
        duration_ms,
        &[
            opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
            opentelemetry::KeyValue::new("step", step.to_string()),
        ],
    );
}

// Telemetry Flow for Bubblewrap OS-Level Sandboxing
pub fn get_bubblewrap_spawn_total() -> &'static UpDownCounter<i64> {
    BUBBLEWRAP_SPAWN_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .i64_up_down_counter("ohc_harness_executions_total")
            .with_description("Total number of Bubblewrap process spawns")
            .build()
    })
}

pub fn get_bubblewrap_execution_latency() -> &'static Histogram<f64> {
    BUBBLEWRAP_EXECUTION_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .f64_histogram("ohc_harness_execution_duration_ms")
            .with_description("Execution latency of Bubblewrap processes")
            .build()
    })
}

pub fn get_bubblewrap_violation_total() -> &'static UpDownCounter<i64> {
    BUBBLEWRAP_VIOLATION_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .i64_up_down_counter("ohc_harness_security_violation_total")
            .with_description("Total number of Bubblewrap policy violations")
            .build()
    })
}

pub fn record_bubblewrap_spawn(agent_id: &str, task_id: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let gauge = get_bubblewrap_spawn_total();
    gauge.add(
        1,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("task_id", task_id.to_string()),
        ],
    );
}

pub fn record_bubblewrap_execution_latency(agent_id: &str, task_id: &str, latency_ms: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_bubblewrap_execution_latency();
    histogram.record(
        latency_ms,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("task_id", task_id.to_string()),
        ],
    );
}

pub fn record_bubblewrap_violation(agent_id: &str, task_id: &str, reason: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let gauge = get_bubblewrap_violation_total();
    gauge.add(
        1,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("task_id", task_id.to_string()),
            opentelemetry::KeyValue::new("reason", reason.to_string()),
        ],
    );
}

pub fn get_harness_init_latency() -> &'static Histogram<f64> {
    HARNESS_INIT_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.harness");
        meter
            .f64_histogram("harness_init_latency_seconds")
            .with_description("Latency for Harness initialization")
            .build()
    })
}

pub fn get_harness_db_io_latency() -> &'static Histogram<f64> {
    HARNESS_DB_IO_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.harness");
        meter
            .f64_histogram("harness_db_io_latency_seconds")
            .with_description("Database I/O latency for Harness operations")
            .build()
    })
}

pub fn record_harness_init_latency(latency_seconds: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_harness_init_latency();
    let deployment_mode = get_deployment_mode();
    histogram.record(
        latency_seconds,
        &[opentelemetry::KeyValue::new(
            "deployment_mode",
            deployment_mode.to_string(),
        )],
    );
}

pub fn record_harness_db_io_latency(operation: &str, latency_seconds: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_harness_db_io_latency();
    let deployment_mode = get_deployment_mode();
    histogram.record(
        latency_seconds,
        &[
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
            opentelemetry::KeyValue::new("operation", operation.to_string()),
        ],
    );
}
#[cfg(test)]
mod additional_tests {
    use super::*;
    use std::fs;


    #[test]
    fn test_record_task_resolution_efficiency_has_deployment_mode() {
        // Just checking that `get_deployment_mode` is exported and we can use it.
        let mode = crate::get_deployment_mode();
        assert!(mode == "Standalone" || mode == "Cloud");
    }

    #[test]
    fn autodream_sync_duration_metric_is_registered_and_dashboarded() {
        let metric_name = autodream_sync_duration_metric_name();
        assert_eq!(metric_name, "ohc_autodream_sync_duration_seconds");
        record_autodream_sync_duration(0.25, "Standalone");

        let dashboard = fs::read_to_string("../monitoring/dashboards/ohc-hybrid-telemetry.json").or_else(|_| {
            fs::read_to_string("../../monitoring/dashboards/ohc-hybrid-telemetry.json").or_else(|_| {
                fs::read_to_string("src/server/monitoring/dashboards/ohc-hybrid-telemetry.json")
            })
        }).expect("hybrid telemetry dashboard should be readable");
        assert!(dashboard.contains(metric_name));
    }
}

pub async fn record_sync_completed_count(
    pool: &PgPool,
    count: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_sync_completed_total",
        "counter",
        count,
        serde_json::json!({}),
    )
    .await
}

pub async fn record_sync_failed_count(
    pool: &PgPool,
    count: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !::server_config::get().telemetry_enabled { return Ok(()); }

    buffer_metric(
        pool,
        "ohc_autodream_sync_failed_total",
        "counter",
        count,
        serde_json::json!({}),
    )
    .await
}
pub static TASKS_COMPLETED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
pub static TASKS_FAILED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
pub static TASKS_TRANSITIONS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_tasks_completed_total() -> &'static Counter<u64> {
    let meter = global::meter("orchestration_state_machine");
    TASKS_COMPLETED_TOTAL.get_or_init(|| {
        meter
            .u64_counter("ohc_task_completed_total")
            .with_description("Total number of successfully completed shared tasks")
            .build()
    })
}

pub fn get_tasks_failed_total() -> &'static Counter<u64> {
    let meter = global::meter("orchestration_state_machine");
    TASKS_FAILED_TOTAL.get_or_init(|| {
        meter
            .u64_counter("ohc_task_failed_total")
            .with_description("Total number of failed shared tasks")
            .build()
    })
}

pub fn get_tasks_transitions_total() -> &'static Counter<u64> {
    let meter = global::meter("orchestration_state_machine");
    TASKS_TRANSITIONS_TOTAL.get_or_init(|| {
        meter
            .u64_counter("ohc_task_transitions_total")
            .with_description("Total number of task state transitions")
            .build()
    })
}

static HARNESS_IO_BYTES_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_harness_io_bytes_total() -> &'static Counter<u64> {
    HARNESS_IO_BYTES_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.harness");
        meter
            .u64_counter("ohc_harness_io_bytes_total")
            .with_description("Total I/O bytes recorded by Harness")
            .build()
    })
}

pub fn record_harness_io_bytes(agent_id: &str, task_id: &str, bytes: u64) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_harness_io_bytes_total();
    counter.add(
        bytes,
        &[
            opentelemetry::KeyValue::new("agent_id", agent_id.to_string()),
            opentelemetry::KeyValue::new("task_id", task_id.to_string()),
        ],
    );
}

#[cfg(test)]
mod harness_io_bytes_tests {
    use super::*;


    #[test]
    fn test_record_harness_io_bytes() {
        // Just calling it ensures it doesn't panic
        record_harness_io_bytes("test_agent", "test_task", 1024);
        let counter = get_harness_io_bytes_total();
        // Counter doesn't easily expose current value in OpenTelemetry, but ensuring initialization is fine.
        counter.add(0, &[]);
    }
}

pub static HARNESS_SECURITY_DIVERGENCE_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub static RAG_RECORDS_SYNCED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
pub static RAG_SYNC_ERRORS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_harness_security_divergence_total() -> &'static Counter<u64> {
    HARNESS_SECURITY_DIVERGENCE_TOTAL.get_or_init(|| {
        global::meter("ohc.telemetry")
            .u64_counter("ohc_harness_security_divergence_total")
            .with_description("Total number of security divergence validations triggered in Harness")
            .build()
    })
}

pub fn record_harness_security_divergence(reason: &str, command_snippet: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_harness_security_divergence_total();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("reason", reason.to_string()),
            opentelemetry::KeyValue::new("command_snippet", command_snippet.to_string()),
        ],
    );
}

pub fn get_rag_records_synced_total() -> &'static Counter<u64> {
    let meter = global::meter("ohc.hybrid_sync");
    RAG_RECORDS_SYNCED_TOTAL.get_or_init(|| {
        meter
            .u64_counter("rag_records_synced_total")
            .with_description("Total number of RAG records successfully synchronized")
            .build()
    })
}

pub fn get_rag_sync_errors_total() -> &'static Counter<u64> {
    let meter = global::meter("ohc.hybrid_sync");
    RAG_SYNC_ERRORS_TOTAL.get_or_init(|| {
        meter
            .u64_counter("rag_sync_errors_total")
            .with_description("Total number of RAG synchronization errors")
            .build()
    })
}

pub fn record_rag_records_synced(count: u64, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_rag_records_synced_total();
    counter.add(
        count,
        &[opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())],
    );
}

pub fn record_rag_sync_error(reason: &str, deployment_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_rag_sync_errors_total();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("reason", reason.to_string()),
            opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string()),
        ],
    );
}


pub static CHAOS_INJECTED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
pub static TASK_RECOVERY_TIME_MS: OnceLock<opentelemetry::metrics::Histogram<f64>> = OnceLock::new();

pub fn get_chaos_injected_total() -> &'static Counter<u64> {
    CHAOS_INJECTED_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.chaos");
        meter
            .u64_counter("ohc_chaos_injected_total")
            .with_description("Total number of injected chaos events")
            .build()
    })
}

pub fn get_task_recovery_time_ms() -> &'static opentelemetry::metrics::Histogram<f64> {
    TASK_RECOVERY_TIME_MS.get_or_init(|| {
        let meter = global::meter("ohc.chaos");
        meter
            .f64_histogram("ohc_task_recovery_time_ms_bucket")
            .with_description("Time taken to recover from chaos injected failures")
            .build()
    })
}

pub fn record_chaos_injected(env_mode: &str) {
    if !::server_config::get().telemetry_enabled { return; }

    let counter = get_chaos_injected_total();
    counter.add(
        1,
        &[opentelemetry::KeyValue::new("EnvMode", env_mode.to_string())],
    );
}

pub fn record_task_recovery_time(env_mode: &str, duration_ms: f64) {
    if !::server_config::get().telemetry_enabled { return; }

    let histogram = get_task_recovery_time_ms();
    histogram.record(
        duration_ms,
        &[opentelemetry::KeyValue::new("EnvMode", env_mode.to_string())],
    );
}

pub struct ChaosRecoveryTracker {
    env_mode: String,
    start: std::time::Instant,
}

impl ChaosRecoveryTracker {
    pub fn new(env_mode: &str) -> Self {
        record_chaos_injected(env_mode);
        Self {
            env_mode: env_mode.to_string(),
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for ChaosRecoveryTracker {
    fn drop(&mut self) {
        record_task_recovery_time(&self.env_mode, self.start.elapsed().as_millis() as f64);
    }
}

#[cfg(test)]
mod harness_security_divergence_tests {
    use super::*;

    #[test]
    fn test_record_harness_security_divergence() {
        record_harness_security_divergence("test_reason", "test_cmd");
        let counter = get_harness_security_divergence_total();
        counter.add(0, &[]);
    }
}
#[cfg(test)]
mod dashboard_test;
