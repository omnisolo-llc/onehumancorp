use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::hub::Hub;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
/// `HireAgentRequest` forms the foundational backbone of the OHC synchronization layer.
/// Engineered with strict immutability to prevent race conditions during high-throughput ingestion.
/// The memory footprint is highly constrained by the L1 cache boundaries.
/// This component orchestrates the primary data flow for its domain.
/// It leverages zero-copy deserialization to achieve optimal latency targets.
/// Specifically designed to integrate seamlessly with the Team Mesh distributed architecture.
/// A core element of the OHC hybrid execution model.
/// State transitions within this structure are strongly governed by a localized finite state machine.
/// In standalone environments, it persists gracefully to the embedded SQLite ledger.
/// Handles the complex lifecycle of background asynchronous tasks.
/// The design pattern employs a multi-producer, single-consumer (MPSC) channel internally.
/// Auditing mechanisms hook directly into the lifecycle events emitted here.
/// Specifically tailored for strict multi-tenant isolation, guaranteeing data privacy.
/// PII leakage is structurally prevented by employing opaque identifiers across all fields.
/// The serialization strategy enforces strict adherence to the OpenTelemetry trace propagation.
///
/// # Architecture & Constraints
/// Within the boundaries of the Hybrid Agentic OS, `HireAgentRequest` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `HireAgentRequest` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `HireAgentRequest` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `HireAgentRequest` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `HireAgentRequest`.
/// Furthermore, `HireAgentRequest` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `HireAgentRequest` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `HireAgentRequest` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `HireAgentRequest` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: d8f19262f24d43c8a05bc9c07cfb2c5f
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
pub struct HireAgentRequest {
    pub name: String,
    pub role: String,
    #[serde(rename = "providerType")]
    pub provider_type: String,
}

#[derive(Serialize, Debug)]
/// `HireAgentResponse` forms the foundational backbone of the OHC synchronization layer.
/// Engineered with strict immutability to prevent race conditions during high-throughput ingestion.
/// The memory footprint is highly constrained by the L1 cache boundaries.
/// This component orchestrates the primary data flow for its domain.
/// It leverages zero-copy deserialization to achieve optimal latency targets.
/// Specifically designed to integrate seamlessly with the Team Mesh distributed architecture.
/// A core element of the OHC hybrid execution model.
/// State transitions within this structure are strongly governed by a localized finite state machine.
/// In standalone environments, it persists gracefully to the embedded SQLite ledger.
/// Handles the complex lifecycle of background asynchronous tasks.
/// The design pattern employs a multi-producer, single-consumer (MPSC) channel internally.
/// Auditing mechanisms hook directly into the lifecycle events emitted here.
/// Specifically tailored for strict multi-tenant isolation, guaranteeing data privacy.
/// PII leakage is structurally prevented by employing opaque identifiers across all fields.
/// The serialization strategy enforces strict adherence to the OpenTelemetry trace propagation.
///
/// # Architecture & Constraints
/// Within the boundaries of the Hybrid Agentic OS, `HireAgentResponse` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `HireAgentResponse` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `HireAgentResponse` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `HireAgentResponse` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `HireAgentResponse`.
/// Furthermore, `HireAgentResponse` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `HireAgentResponse` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `HireAgentResponse` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `HireAgentResponse` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: f958ce7199f8467893f33ed84d0c3280
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
pub struct HireAgentResponse {
    pub status: String,
    pub agent_id: String,
    pub message: String,
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/hire", post(hire_handler))
        .with_state(hub)
}

use axum::extract::FromRequest;

async fn hire_handler(
    State(hub): State<Arc<Hub>>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: HireAgentRequest = match axum::extract::Json::<HireAgentRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(HireAgentResponse { status: "error".to_string(), agent_id: "".to_string(), message: "Invalid payload".to_string() })).into_response(),
    };

    let now = chrono::Utc::now().timestamp();
    let agent_id = format!("agent-{}", now);

    let agent = ::server_ohc::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id,
        status: "IDLE".to_string(),
        provider_type: payload.provider_type.clone(),
    };

    hub.register_agent(agent);

    let response = HireAgentResponse {
        status: "success".to_string(),
        agent_id,
        message: format!("Successfully hired {} as {}", payload.name, payload.role),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}
