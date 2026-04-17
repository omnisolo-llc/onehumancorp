<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Edge LLM Offloading Protocol API

This playbook documents the API endpoint and MCP tool configuration for the Edge LLM Offloading Protocol.

## Overview

The `mcp_inference_router` tool dynamically evaluates a prompt's token count and the local Standalone instance's resource utilization. Depending on the size of the prompt and the presence of privacy flags, it seamlessly routes inference requests either locally (via SQLite cache) or to the multi-tenant K8s cloud backend.

## API Endpoint: `POST /api/v1/inference/route`

This endpoint accepts a standard text completion request payload, assesses the prompt complexity, and returns the appropriately routed response.

**Request Payload:**

```json
{
  "prompt": "Evaluate the current architectural constraints of the swarm orchestration.",
  "max_tokens": 1500,
  "is_sensitive": false,
  "metrics": {
    "local_cpu_utilization": 85.0,
    "available_memory_mb": 512
  }
}
```

- **`is_sensitive`** (`boolean`): If `true`, the request is strictly enforced to run locally, regardless of the prompt size or local resource constraints.
- **`metrics`**: Optional local metrics sent by the standalone client. If omitted, the router will query local system state dynamically.

**Response Payload (Routed to Cloud):**

```json
{
  "status": "success",
  "routed_to": "cloud-assisted",
  "completion": {
    "text": "The swarm orchestration heavily relies on... [Cloud inferred response]",
    "tokens_used": 1450,
    "latency_ms": 120
  }
}
```

**Response Payload (Routed Locally):**

```json
{
  "status": "success",
  "routed_to": "local",
  "completion": {
    "text": "Based on local cache and inference: The orchestration...",
    "tokens_used": 1450,
    "latency_ms": 350
  }
}
```

</div>
