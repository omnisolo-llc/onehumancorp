---
status: IN_PROGRESS
agent: Researcher
---

# Title: Observability Gap Analysis: Missing AutoDream and Mesh Telemetry

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) backend exposes OpenTelemetry metrics for AutoDream (e.g., `ohc_autodream_sync_duration_seconds`, `ohc_autodream_query_duration_seconds`) and Teammate Mesh operations (e.g., `ohc_mesh_latency`, `ohc_mesh_broadcast_total`). However, these metrics are not visualized in the Grafana dashboard (`hybrid-telemetry.json`), resulting in an observability gap that obscures bottlenecks in memory consolidation and inter-agent communication.

## Research Report
An audit of `srcs/server/telemetry/telemetry.go` confirms the existence of the following metrics:
- `ohc_autodream_sync_duration_seconds`
- `ohc_autodream_query_duration_seconds`
- `ohc_mesh_latency`
- `ohc_mesh_broadcast_total`
A search of `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` reveals that none of these metrics are currently plotted. In a hybrid setup, observing these latency and broadcast throughput metrics is critical for diagnosing performance issues.

## Design Doc
1. **Grafana Dashboards Update**:
   - Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
   - Add a new panel for "AutoDream Sync Duration" tracking `ohc_autodream_sync_duration_seconds`.
   - Add a new panel for "AutoDream Query Duration" tracking `ohc_autodream_query_duration_seconds`.
   - Add a new panel for "Teammate Mesh Latency" tracking `ohc_mesh_latency`.
   - Add a new panel for "Teammate Mesh Broadcasts" tracking `ohc_mesh_broadcast_total`.
   - Ensure these panels match the JSON configuration of existing native Grafana timeseries panels.

## Implementation Prompt
Hello Implementer agent! Please execute the following tasks:
1. Open `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. Add panels for the four missing metrics: `ohc_autodream_sync_duration_seconds`, `ohc_autodream_query_duration_seconds`, `ohc_mesh_latency`, and `ohc_mesh_broadcast_total`.
3. Verify your JSON syntax by running `bazelisk test //srcs/server/...` to ensure no embedded tests break.

## Priority
P1

## Estimated Scope
Medium
