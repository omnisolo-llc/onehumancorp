---
status: DONE
agent: Maintainer
---

# Title: Implement Standalone Metric Buffering & Cloud Sync

## Problem Statement
OHC's Standalone Desktop mode (SQLite) lacks robust observability compared to our Cloud deployment. Local agent executions (token usage, latency) are not consistently tracked because the standard Prometheus push models assume a persistent connection to the metrics server. We need a way to buffer these metrics locally in SQLite and periodically sync them to the OHC Cloud, ensuring full Swarm Intelligence observability without compromising data privacy.

## Research Report
The `RESEARCH_REPORT_STANDALONE_METRICS.md` audit identifies this "Hybrid Observability" gap. Competitors (Claude Code) ignore local telemetry, while others (OpenClaw) require always-on cloud connections. OHC will implement a persistent local buffer with background batch syncing, protected by strict PII scrubbing.

## Design Doc
1. **Local Metric Buffer**: Create a new SQLite table `local_metrics_buffer` to store raw telemetry events.
2. **PII Redaction**: All structured metric payloads sent to `telemetry.BufferMetricFunc` must be deeply scrubbed using `telemetry.RedactInterfacePII` prior to buffering.
3. **Background Sync Daemon**: Implement a background worker in Standalone mode that periodically queries `local_metrics_buffer`, sends a batch to the Cloud API, and deletes the synced rows.
4. **Cloud API Endpoint**: Add `POST /api/telemetry/sync` to the Cloud API to receive and ingest these batches into the central Prometheus/Postgres store.

## Implementation Prompt
Hello Implementer agent! Please build the Standalone Metric Buffering system.
1. Review the existing telemetry setup in `srcs/server/telemetry/`.
2. Implement the `BufferMetricFunc` ensuring it calls `telemetry.RedactInterfacePII` before writing to the local SQLite database.
3. Create the background sync daemon `srcs/server/telemetry/sync_daemon.go`.
4. Implement the Cloud API receiver `POST /api/telemetry/sync`.
5. Write unit tests ensuring PII is correctly redacted before buffering.

## Priority
P1

## Estimated Scope
Medium
