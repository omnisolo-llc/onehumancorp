# Research Report: Hybrid Telemetry Sync Gap Analysis

## Title
Implement Hybrid Telemetry Buffering for MCP Tool Execution

## Problem Statement
The OneHumanCorp platform relies on an accurate global perspective of AI workforce resource usage for billing, quota limits, and optimization analysis. While `telemetry.McpSyncWorker` accurately tracks MCP tool executions in Cloud-native deployments via `ohc_mcp_tool_calls_total`, this metric lacks offline Standalone mode buffering. As a result, when Standalone desktop users operate in offline environments, the `telemetry_buffer` fails to capture their local tool executions, leading to under-reported metrics once connectivity is restored. From a business owner's perspective, this causes missing analytics in their financial dashboard, obscuring true agent utilization. From a Swarm Operator's perspective, this results in incomplete cross-mode optimization and telemetry gaps.

## Research Report
*   **Discovery Findings**: Our discovery focused on `srcs/server/telemetry/`. We reviewed Prometheus metric definitions (`mcp_metrics.go`), offline sync handlers (`mcp_sync_worker.go`, `sync_worker.go`), and core telemetry instrumentations (`telemetry.go`).
*   **Gap Identification**: Throughout the core telemetry system (e.g., `RecordTokenUsage`, `RecordRAGRecordsSynced`), functions implement a standard check using the local buffer function. However, the MCP tool calls counter in `mcp_metrics.go` is defined but missing a dedicated recording wrapper that performs this buffer injection. Currently, it seems this metric is either recorded inline without buffering or omitted from local capture entirely.
*   **Metrics Review**: This inconsistency manifests in different throughputs between Standalone and Cloud-native modes for MCP-driven operations, as Cloud direct Prometheus scraping succeeds, whereas Standalone offline usage is lost before it can be batch synced by `McpSyncWorker` to the `/api/telemetry/sync` endpoint.
*   **Impact**: Loss of tool execution telemetry metrics during local/standalone execution contexts.

## Design Doc
*   **Entity Modifications**: Introduce a centralized recording wrapper for MCP tool calls within the telemetry module.
*   **Integration Points**:
    *   Create a method to record tool calls and increment the standard Prometheus counter.
    *   Check for the presence of the standalone buffer. If present, serialize the necessary payload, pass it through the required PII redaction mechanism, and buffer it.
*   **Architecture Flow**:
    1. Agent invokes tool locally.
    2. Operation calls the new telemetry recording method.
    3. If offline/standalone, data persists into local SQLite via the buffer mechanism.
    4. Upon network restoration, `McpSyncWorker` pushes this payload to the cloud API endpoint via SPIFFE mTLS.

## Implementation Prompt
Implement the telemetry recording function for MCP tool calls to guarantee offline synchronization parity between Standalone and Cloud-native modes.
1. In `srcs/server/telemetry/mcp_metrics.go`, define a new exported function to record tool calls and statuses.
2. Ensure this function increments the existing `MCPToolCallsTotal` prometheus counter.
3. Ensure this function buffers the metric type `"ohc_mcp_tool_calls_total"` and a JSON payload containing tool name and status if the standalone buffer is enabled.
4. The JSON payload MUST be passed through the standard PII redaction layer before being marshalled and written to the buffer to comply with privacy requirements.
5. Create an accompanying unit test asserting that the standalone buffer is triggered correctly during standalone execution contexts.

## Priority
P1

## Estimated Scope
Small
