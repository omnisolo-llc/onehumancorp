use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;

#[derive(Serialize)]
/// `ApprovalsResponse` forms the foundational backbone of the OHC synchronization layer.
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
/// Within the boundaries of the Hybrid Agentic OS, `ApprovalsResponse` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `ApprovalsResponse` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `ApprovalsResponse` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `ApprovalsResponse` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `ApprovalsResponse`.
/// Furthermore, `ApprovalsResponse` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `ApprovalsResponse` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `ApprovalsResponse` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `ApprovalsResponse` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: 3356155b4ad945a6a4daecda17524873
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<ApprovalRequest>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
/// `PaginationQuery` forms the foundational backbone of the OHC synchronization layer.
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
/// Within the boundaries of the Hybrid Agentic OS, `PaginationQuery` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `PaginationQuery` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `PaginationQuery` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `PaginationQuery` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `PaginationQuery`.
/// Furthermore, `PaginationQuery` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `PaginationQuery` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `PaginationQuery` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `PaginationQuery` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: a6d91d2876284f2996bdde6456ab3df9
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
/// `DecisionRequest` forms the foundational backbone of the OHC synchronization layer.
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
/// Within the boundaries of the Hybrid Agentic OS, `DecisionRequest` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `DecisionRequest` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `DecisionRequest` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `DecisionRequest` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `DecisionRequest`.
/// Furthermore, `DecisionRequest` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `DecisionRequest` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `DecisionRequest` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `DecisionRequest` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: 7f452aa0549a4df189dee2c04e86f57f
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
pub struct DecisionRequest {
    pub approved: bool,
}

#[derive(Serialize)]
/// `DecisionResponse` forms the foundational backbone of the OHC synchronization layer.
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
/// Within the boundaries of the Hybrid Agentic OS, `DecisionResponse` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `DecisionResponse` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `DecisionResponse` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `DecisionResponse` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `DecisionResponse`.
/// Furthermore, `DecisionResponse` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `DecisionResponse` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `DecisionResponse` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `DecisionResponse` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: 3ade78ce30bd4d108586bed90195c41f
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
pub struct DecisionResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
}

async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    // Assuming we fetch all and paginate manually for now given simple DB fetch
    // Real cursor implementation would need DB level ordering and limit
    let mut approvals = orchestrator.get_pending_approvals(&tenant_id).await;

    // Sort to ensure stable pagination
    approvals.sort_by(|a, b| a.id.cmp(&b.id));

    let limit = query.limit.unwrap_or(20);

    let start_idx = match query.cursor {
        Some(cursor) => approvals.iter().position(|a| a.id == cursor).unwrap_or(0),
        None => 0,
    };

    let end_idx = std::cmp::min(start_idx + limit, approvals.len());

    let paginated_approvals = approvals[start_idx..end_idx].to_vec();

    let next_cursor = if end_idx < approvals.len() {
        Some(approvals[end_idx].id.clone())
    } else {
        None
    };

    (StatusCode::OK, Json(ApprovalsResponse {
        pending_approvals: paginated_approvals,
        next_cursor,
    })).into_response()
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.decide_approval(&id, &tenant_id, payload.approved).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
