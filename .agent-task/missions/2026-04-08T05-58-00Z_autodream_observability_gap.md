---
status: PENDING
agent: Implementer
---

# AutoDream & Hybrid Sync Observability Panels

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) seamlessly bridges local Standalone Desktop execution with multi-tenant Cloud orchestration via Swarm Intelligence Protocol (OHC-SIP). A critical component of this is the background "Offline-to-Cloud State Sync" and the "AutoDream Memory Ingestion". However, an observability audit reveals that critical telemetry emitted by the backend to track the health, latency, and throughput of these sync processes is missing from Grafana visualization (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`).

## Research Report
An automated test of backend Prometheus metric definitions against our Grafana JSON dashboard provisioning files identified multiple unmapped metrics critical to monitoring hybrid sync bottlenecks:
- **AutoDream Sync Latency:** `ohc_autodream_sync_duration_seconds`
- **AutoDream Query Latency:** `ohc_autodream_query_duration_seconds`
- **General Hybrid Sync Health:** `sync_completed_count`, `sync_failed_count`
- **Sync Batching Throughput:** `sync_daemon_batch_size`

Without these metrics on our dashboards, we operate with a blind spot regarding the performance of our "Local-Private RAG with Cloud-Scale Routing" features.

## Design Doc
1. **Grafana Dashboards Update**:
   - Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include the unmapped metrics.
   - **AutoDream Operation Latency Panel**: A timeseries panel displaying P95 latency for `ohc_autodream_sync_duration_seconds` and `ohc_autodream_query_duration_seconds`.
   - **Hybrid Sync Status Panel**: A timeseries or bar gauge tracking `sync_completed_count` vs. `sync_failed_count` to proactively alert on offline buffer sync failures.
   - **Sync Batch Size Panel**: A timeseries panel tracking the `sync_daemon_batch_size` histogram.
2. **Dashboard Verification**:
   - Ensure native Grafana standard JSON configurations match the existing Dark Mode theme.

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. Navigate to `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. Add a new panel for **AutoDream Operation Latency** targeting `ohc_autodream_sync_duration_seconds` and `ohc_autodream_query_duration_seconds`.
3. Add a new panel for **Hybrid Sync Status** tracking `sync_completed_count` and `sync_failed_count`.
4. Add a new panel for **Sync Batch Size** tracking `sync_daemon_batch_size`.
5. Use `bazelisk test //srcs/server/...` to ensure your JSON syntax hasn't broken any embed tests and that telemetry integration points are still correct.
