# Hybrid MCP API Playbook

This playbook defines the Model Context Protocol (MCP) interactions required for the OHC Hybrid integration layer. The primary components are the Telemetry-MCP Bridge and the Standalone Sync Proxy.

## Telemetry-MCP Bridge Endpoints

### 1. `telemetry.query`
Fetches specific correlated hybrid metrics from both Cloud and Standalone environments.

**Request Payload:**
```json
{
  "method": "telemetry.query",
  "params": {
    "metrics": ["ohc_autodream_rag_latency_seconds", "ohc_standalone_sync_queue_size"],
    "time_range": "1h",
    "environment": "hybrid"
  }
}
```

**Response Payload:**
```json
{
  "result": {
    "data": [
      {
        "metric": "ohc_autodream_rag_latency_seconds",
        "values": [[1712345678, "0.45"]]
      }
    ]
  }
}
```

## Standalone Sync Proxy Contracts
The proxy intercepts external MCP tool calls when disconnected and buffers them in `hybrid_mcp_sync_queue` table locally.

### Buffered State Object
```json
{
  "tool_name": "jira_create_issue",
  "buffered_at": "2024-04-14T12:00:00Z",
  "payload": { ... },
  "status": "pending_sync"
}
```
