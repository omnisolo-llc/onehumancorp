<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Swarm-Aware MCP Telemetry Mesh Walkthrough

Welcome to the Telemetry Mesh guide. This walkthrough explains how the OHC Hybrid Architecture securely synchronizes observability metrics from local Standalone deployments to the Cloud PostgreSQL/Prometheus stack.

## 1. Observability Sync Lifecycle

Agents running in Standalone Mode log metrics locally to the SQLite SIPDB. The Telemetry Mesh batches and transmits these over a secure SPIFFE/SPIRE authenticated channel when connected.

```mermaid
sequenceDiagram
    participant Agent as Local Agent
    participant SQLite as Local SIPDB
    participant TelemetryMesh as MCP Telemetry Mesh
    participant Gateway as Cloud API Gateway
    participant Prom as Cloud Prometheus/pgvector

    Agent->>SQLite: 1. Record Local Metrics
    TelemetryMesh->>SQLite: 2. Read Buffered Metrics
    TelemetryMesh->>Gateway: 3. Sync Batch (mTLS / SPIFFE)
    Gateway->>Prom: 4. Ingest into Global Telemetry
    Gateway-->>TelemetryMesh: 5. ACK Sync
    TelemetryMesh->>SQLite: 6. Flush Synced Buffer
```

## 2. Telemetry MCP Operations

The Telemetry MCP provides endpoints for agents to query their own health metrics autonomously, validating multitenant isolation.

For full API specifications, please refer to the [API Playbook](../api/playbook.md).

</div>
