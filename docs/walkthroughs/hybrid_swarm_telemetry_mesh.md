<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Swarm-Aware Telemetry Mesh: Visual Walkthrough

This guide explains how OHC ensures Full-Spectrum Observability by capturing telemetry in Standalone Mode and synchronizing it securely with Cloud metrics.

## 1. Zero-Trust Telemetry Pipeline

The telemetry mesh leverages SPIFFE/SPIRE for mTLS validation when transferring metrics from the local SQLite buffer to the Cloud Postgres cluster.

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

## 2. Sync Execution

When connectivity is restored, the `McpSyncWorker` reads the `telemetry_buffer` table, pushes the batch of metrics securely, and marks them as synced to prevent deduplication errors.

</div>
