# [observability] Swarm Health & Hybrid Telemetry Observability Gap Analysis

## Problem Statement
The OHC platform lacks comprehensive, mode-specific (Cloud vs. Standalone) observability across several critical subsystems. While OpenTelemetry and Prometheus are implemented for basic API metrics, deep operational visibility is missing for KAIROS state machine transitions, hybrid database operations, sync daemon performance, and AutoDream memory consolidation. This prevents swarm operators and human CEOs from diagnosing job queue depths, local SQLite bottlenecks, and synchronization errors.

## Research Report
An audit of the OHC telemetry mesh reveals the following gaps:

1. **KAIROS Orchestrator Dashboard Gap**: The `kairos` module tracks `TransitionsTotal`, `TransitionDuration`, and `TaskQueueDepth`, but there is no premium Grafana dashboard (`monitoring/dashboards/kairos_dashboard.json`) visualizing these metrics grouped by `mode`.
2. **Hybrid Database Metrics Gap**: The backend exports `db.client.operation.duration`, `db.client.operation.errors`, and `sqlite_lock_contention`. However, `monitoring/dashboards/database_metrics.json` does not exist, leaving no UI to track Postgres (Cloud) vs. SQLite (Standalone) throughput and lock contention.
3. **AutoDream Memory Consolidation Gap**: The AutoDream worker (`src/server/orchestration/autodream.go` and `autodream_worker.go`) is a black box. It lacks Prometheus metrics for memory processing latency, batch sizes, and consolidation errors.
4. **Sync Daemon Metrics Gap**: The `HybridMCPRAGDaemon` in `sync_daemon.go` syncs local agent missions to the cloud in Standalone mode but lacks mode-specific metric labeling and explicit error/latency tracking, making offline-to-online sync reliability opaque.

## Design Doc
To address these gaps, we propose a comprehensive telemetry mesh expansion:

1. **New Dashboards**:
   - `monitoring/dashboards/kairos_dashboard.json`: Visualizes task queue depth, state machine transition rates, and transition durations by mode.
   - `monitoring/dashboards/database_metrics.json`: Visualizes total query latency, error rate by operation, and SQLite lock contention.
   - *UI Requirement*: All dashboards must include an HTML/Text panel injecting the OHC Premium CSS tokens (`backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif;`).

2. **New AutoDream Metrics**:
   - Introduce `src/server/orchestration/autodream/metrics.go` to define `MemoriesProcessedTotal`, `BatchProcessingDuration`, and `ConsolidationErrorsTotal` (Counter/Histogram).
   - Instrument `autodream_worker.go` to increment these metrics.

3. **Enhanced Sync Daemon Metrics**:
   - Update telemetry methods in `sync_daemon.go` to include a `mode` label (Standalone vs Cloud).
   - Add a `SyncDaemonErrorTotal` Counter and track sync latency (P95) and batch sizes more explicitly.
   - Expand `kairos_hybrid_metrics.json` with panels for these sync indicators.

## Implementation Prompt
1. Create Grafana dashboards `kairos_dashboard.json` and `database_metrics.json` with the required metrics and premium CSS styling.
2. Implement new Prometheus metrics for the AutoDream pipeline in `src/server/orchestration/autodream/metrics.go` and instrument the worker.
3. Enhance the `sync_daemon.go` telemetry calls to include mode labels and implement a new `SyncDaemonErrorTotal` metric. Update `kairos_hybrid_metrics.json` to reflect these changes.
4. Ensure 100% unit test coverage for the new metric instrumentations.

## Priority
P1

## Estimated Scope
Medium
