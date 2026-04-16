# Hybrid MCP Integration API Playbook

This playbook documents the payload structure and gRPC/MCP client endpoints for the `telemetry-mcp-bridge` component of the KAIROS Orchestrator.

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

## 1. Telemetry-MCP Bridge Endpoints

The `telemetry-mcp-bridge` exposes standard MCP (Model Context Protocol) JSON-RPC endpoints over HTTP/WebSocket or Stdio to sync telemetry events between standalone and cloud environments.

### `mcp.telemetry.sync`

Synchronizes a batch of local telemetry metrics to the OHC Central Database via the KAIROS Orchestrator.

**Method:** `tools/call`

**Request Payload (`params`):**

```json
{
  "name": "sync_telemetry",
  "arguments": {
    "node_id": "standalone-node-1234",
    "timestamp": "2026-04-16T12:00:00Z",
    "metrics": [
      {
        "name": "ohc_agent_inference_ms",
        "value": 150.5,
        "labels": {"agent": "Scribe", "model": "claude-4"}
      },
      {
        "name": "ohc_sync_queue_size",
        "value": 5,
        "labels": {"queue": "telemetry"}
      }
    ]
  }
}
```

**Response Payload (`result`):**

```json
{
  "content": [
    {
      "type": "text",
      "text": "Telemetry batch synced successfully. 2 metrics processed."
    }
  ],
  "isError": false
}
```

## 2. Resource Endpoints

Exposes active system metrics and health probes to orchestration agents.

### `mcp.resources.list`

Lists available observability resources for the current node.

**Method:** `resources/list`

**Response Payload (`result`):**

```json
{
  "resources": [
    {
      "uri": "ohc://telemetry/health",
      "name": "Node Health Status",
      "mimeType": "application/json"
    },
    {
      "uri": "ohc://telemetry/queue_status",
      "name": "Standalone Sync Queue Status",
      "mimeType": "application/json"
    }
  ]
}
```

</div>
