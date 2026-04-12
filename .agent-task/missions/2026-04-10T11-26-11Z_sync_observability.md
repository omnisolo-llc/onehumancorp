---
status: DONE
agent: jules

---

# Title: Implement Hybrid Synchronization Observability for SIPDB

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) relies heavily on `SyncBufferedMetrics` and `SyncContextSync` inside `srcs/server/orchestration/sip.go` to synchronize local SQLite data to the Cloud Postgres cluster. However, these critical synchronization pipelines lack comprehensive OpenTelemetry/Prometheus metrics, creating an observability gap for synchronization latencies, throughput, and payload sizes across Hybrid nodes.

## Research Report
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

### Market Audit & OHC Advantage

Competitors frequently ignore the observability of offline-to-cloud synchronization. OHC's "Unfair Advantage" is our strict, deterministic synchronization of state across disparate compute environments.

| Feature Area | Claude Code | OpenClaw | **OHC Vision** |
| :--- | :--- | :--- | :--- |
| **Sync Telemetry** | Blind | Minimal | **High-fidelity OpenTelemetry** |
| **Payload Metrics** | None | None | **Byte-level tracking** |

### Visualizing the Data Flow

```mermaid
graph TD
    A[Local SIPDB SQLite] -->|SyncTrigger| B(SyncBufferedMetrics)
    B --> C{Prometheus Metric Wrapper}
    C -->|Records Latency & Size| D[Cloud OTLP / Postgres]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,D premium;
    class B,C premium;
```

</div>

## Design Doc
1. Define new Prometheus metrics in `srcs/server/telemetry/telemetry.go`: `ohc_sync_latency_seconds` and `ohc_sync_payload_bytes`.
2. Wrap the HTTP payload dispatch loops in `SyncBufferedMetrics` and `SyncContextSync` (`srcs/server/orchestration/sip.go`) to calculate the size of JSON payloads synced to the remote endpoint.
3. Call `telemetry.RecordSyncLatency` and `telemetry.RecordSyncPayloadSize` upon successful sync responses.

## Implementation Prompt
Hello Implementer agent! Please add observability tracking to `sip.go` sync loops.
1. Add `RecordSyncLatency(ctx, duration)` and `RecordSyncPayloadSize(ctx, bytes)` telemetry wrappers in `srcs/server/telemetry/telemetry.go`.
2. Integrate these calls into `SyncBufferedMetrics` and `SyncContextSync` in `srcs/server/orchestration/sip.go`.
3. Add unit tests for telemetry metrics update.

## Priority
P1

## Estimated Scope
Small
