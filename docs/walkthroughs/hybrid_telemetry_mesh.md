<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Swarm-Aware MCP Telemetry Mesh Walkthrough

Welcome to the Hybrid Swarm-Aware MCP Telemetry Mesh interactive walkthrough. This guide explains how OHC achieves full-spectrum observability across both Standalone SQLite and Cloud PostgreSQL deployments.

## 1. Architectural Overview

Unlike competitors that lock telemetry within cloud silos or lack persistent telemetry entirely, OHC leverages its Hybrid Architecture (OHC-HA) to seamlessly synchronize local SQLite-based metrics with Cloud PostgreSQL-based observability.

```mermaid
graph TD
    A[Standalone OHC Agent] -->|Logs Metrics locally| B(Local SQLite SIPDB)
    A -->|SPIFFE/SPIRE SVID| C[Local mTLS Proxy]
    B -.->|Background MCP Sync| D{Cloud MCP Gateway}
    C -.->|Auth Handshake| D
    D -->|Aggregates Metrics| E[(Cloud PostgreSQL / Prometheus)]
    E -->|Grafana Visualizations| F[Internal Dashboard]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

## 2. Local Metric Buffering

Agents running in local Standalone Mode log OpenTelemetry metrics to the SQLite SIPDB (`telemetry_buffer` table). This persistent buffer ensures zero data loss when offline.

## 3. Cloud Synchronization

When internet connectivity is restored or cloud scaling is required, the `McpSyncWorker` securely transmits (via SPIFFE/SPIRE identity) these metrics to the Cloud PostgreSQL/Prometheus stack.

</div>
