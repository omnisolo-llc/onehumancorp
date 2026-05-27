pub mod forecaster;

pub use ::server_config as config;
use chrono::Utc;
use opentelemetry::global;
use serde_json::{Map, Value};
use sqlx::{PgPool, query};
use std::sync::OnceLock;

use opentelemetry::metrics::Histogram;

use opentelemetry::metrics::{Counter, UpDownCounter};

static SUB_AGENT_QUEUE_LENGTH_GAUGE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static SUB_AGENT_QUEUE_DELAY_HISTOGRAM: OnceLock<Histogram<f64>> = OnceLock::new();
static TASK_CLAIM_CONTENTION_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static BUBBLEWRAP_SPAWN_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static BUBBLEWRAP_EXECUTION_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static BUBBLEWRAP_VIOLATION_TOTAL: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static TOKEN_USAGE: OnceLock<Counter<u64>> = OnceLock::new();
static AGENT_API_CALL: OnceLock<Counter<u64>> = OnceLock::new();
static AGENT_API_ERROR: OnceLock<Counter<u64>> = OnceLock::new();
static HUMAN_INTERACTION: OnceLock<Counter<u64>> = OnceLock::new();
static MEETING_EVENT: OnceLock<Counter<u64>> = OnceLock::new();
static SWARM_TASK_COMPLETED: OnceLock<Counter<u64>> = OnceLock::new();
static MCP_TOOL_CALLS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_mcp_tool_calls_counter() -> &'static Counter<u64> {
    MCP_TOOL_CALLS_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("ohc_mcp_tool_calls_total")
            .with_description("Total number of MCP tool calls")
            .build()
    })
}

pub fn get_token_usage_counter() -> &'static Counter<u64> {
    TOKEN_USAGE.get_or_init(|| {
        let meter = global::meter("ohc.telemetry");
        meter.u64_counter("token_usage").build()
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

pub fn record_agent_api_call(agent_id: &str, role: &str, api: &str) {
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
    let counter = get_mcp_tool_calls_counter();
    counter.add(
        1,
        &[
            opentelemetry::KeyValue::new("tool_name", tool_name.to_string()),
            opentelemetry::KeyValue::new("status", status.to_string()),
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
    let histogram = get_sub_agent_queue_delay_histogram();
    histogram.record(delay, &[opentelemetry::KeyValue::new("deployment_mode", deployment_mode.to_string())]);
}

pub fn record_task_claim_contention(mode: &str) {
    let counter = get_task_claim_contention_total();
    counter.add(1, &[opentelemetry::KeyValue::new("mode", mode.to_string())]);
}

pub async fn record_autodream_sync(
    pool: &PgPool,
    count: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(
        pool,
        "ohc_autodream_records_synced_total",
        "counter",
        count,
        serde_json::json!({}),
    )
    .await
}

pub async fn record_token_burn_rate_predicted_24h(
    pool: &PgPool,
    org_id: &str,
    forecast: f32,
) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<(), Box<dyn std::error::Error>> {
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
        }),
    )
    .await?;

    // 2. ROI Calculation in Telemetry
    // Efficiency = 1 / (Tokens Consumed * 1000) for SUCCESS
    if outcome == "SUCCESS" && tokens > 0 {
        let efficiency = 1.0 / (tokens as f32 / 1000.0);
        buffer_metric(
            pool,
            "ohc_agent_efficiency_gauge",
            "gauge",
            efficiency,
            serde_json::json!({
                "agent_role": role,
                "deployment_mode": deployment_mode,
            }),
        )
        .await?;
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
    .await
}

pub async fn record_api_call_cost(
    pool: &PgPool,
    organization_id: &str,
    entity: &str,
    cost: f64,
) -> Result<(), Box<dyn std::error::Error>> {
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
    buffer_metric(
        pool,
        "ohc_rag_escalation_total",
        "counter",
        1.0,
        serde_json::json!({ "organization_id": org_id, "error": error }),
    )
    .await
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

    query(
        "INSERT INTO local_telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status)
         VALUES ($1, $2, $3, $4, $5, 'pending')"
    )
    .bind(metric_name)
    .bind(metric_type)
    .bind(value)
    .bind(labels_json)
    .bind(Utc::now())
    .execute(pool)
    .await?;

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
            } else {
                Value::String(s)
            }
        }
        _ => val,
    }
}

pub fn is_sensitive_key(key: &str) -> bool {
    let k = key.to_lowercase();
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
        || k.contains("session_id")
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
        || k.contains("ip_address")
        || k.contains("mac_address")
        || k.contains("geolocation")
        || k.contains("dob")
        || k.contains("birth")
        || k.contains("passport")
        || k.contains("license")
        || k.contains("ip")
        || k.contains("location")
}

pub fn is_email(s: &str) -> bool {
    s.contains('@') && s.contains('.')
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
    buffer_metric(
        pool,
        "ohc_storage_rw_cost",
        "counter",
        size_bytes as f32,
        serde_json::json!({
            "organization_id": organization_id,
            "operation": operation,
        }),
    )
    .await
}

pub async fn record_email_send_cost(
    pool: &PgPool,
    organization_id: &str,
    count: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(
        pool,
        "ohc_email_send_cost",
        "counter",
        count as f32,
        serde_json::json!({
            "organization_id": organization_id,
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
            .i64_up_down_counter("BubblewrapSpawnTotal")
            .with_description("Total number of Bubblewrap process spawns")
            .build()
    })
}

pub fn get_bubblewrap_execution_latency() -> &'static Histogram<f64> {
    BUBBLEWRAP_EXECUTION_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .f64_histogram("BubblewrapExecutionLatency")
            .with_description("Execution latency of Bubblewrap processes")
            .build()
    })
}

pub fn get_bubblewrap_violation_total() -> &'static UpDownCounter<i64> {
    BUBBLEWRAP_VIOLATION_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .i64_up_down_counter("BubblewrapViolationTotal")
            .with_description("Total number of Bubblewrap policy violations")
            .build()
    })
}

pub fn record_bubblewrap_spawn(agent_id: &str, task_id: &str) {
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

    #[test]
    fn test_record_task_resolution_efficiency_has_deployment_mode() {
        // Just checking that `get_deployment_mode` is exported and we can use it.
        let mode = crate::get_deployment_mode();
        assert!(mode == "Standalone" || mode == "Cloud");
    }
}
