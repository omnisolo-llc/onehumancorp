# Observability Gaps Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement identified observability gaps by adding dashboard visualizations and sandbox telemetry metrics.

**Architecture:** Modifies existing JSON Grafana dashboards to include missing panels (`ohc_task_claim_contention_total`, styled database metrics) and adds rust OpenTelemetry code to track sandbox metrics (`ohc_sandbox_cpu_usage`, `ohc_sandbox_memory_bytes`, `ohc_sandbox_network_io`).

**Tech Stack:** Grafana JSON configuration, Rust (OpenTelemetry)

---

### Task 1: Add Sandbox Telemetry Dashboard

**Files:**
- Create: `src/server/monitoring/grafana/dashboards/sandbox_observability.json`

- [ ] **Step 1: Create Dashboard JSON**
  - Create the `sandbox_observability.json` file. It should visualize `ohc_sandbox_cpu_usage`, `ohc_sandbox_memory_bytes`, and `ohc_sandbox_network_io`.
  - Add an HTML/text panel containing the `<style>` block with the glassmorphism CSS `backdrop-filter: blur(30px) saturate(210%)` and `background: rgba(22, 22, 26, 0.7)` (or similar light/dark values from custom.css).
- [ ] **Step 2: Verify Dashboard JSON**
  - Run `cat src/server/monitoring/grafana/dashboards/sandbox_observability.json` to verify the creation and structure.

### Task 2: Implement Sandbox Metrics in Telemetry Code

**Files:**
- Modify: `src/server/telemetry/mod.rs`
- Modify: `src/agents/builtin/sandbox/manager.rs`

- [ ] **Step 1: Define and implement sandbox metrics in telemetry registry**
  - Inside `src/server/telemetry/mod.rs`, add `record_sandbox_cpu_usage`, `record_sandbox_memory_bytes`, and `record_sandbox_network_io` to the telemetry module.
- [ ] **Step 2: Add tests for telemetry functions**
  - Inside the tests module of `src/server/telemetry/mod.rs` (at the end of the file, matching `mod tests`), add a test to verify recording `ohc_sandbox_cpu_usage`, `ohc_sandbox_memory_bytes`, and `ohc_sandbox_network_io`.
- [ ] **Step 3: Call metric recorders from sandbox execution**
  - In `src/agents/builtin/sandbox/manager.rs`, after the `let duration = start_time.elapsed();` around line 160, inject the `crate::telemetry::record_sandbox_*` calls. Provide mock variables `cpu_usage=0.5, memory_bytes=1024, network_io=512` as placeholders for the telemetry since we can't fetch real `rusage` trivially across platforms without more crates. Provide `session_id` as a label instead of missing `agent_id/task_id` since only session ID is stored in the sandbox manager class.
- [ ] **Step 4: Verify test passes**
  - Run `bazelisk test //src/server/telemetry:server_telemetry_unit_test` and `bazelisk test //src/agents/builtin:ohc_builtin_agent_lib_unit_test`.

### Task 3: Update Kairos Hybrid Metrics Dashboard

**Files:**
- Modify: `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json`
- Modify: `deploy/helm/ohc/dashboards/kairos_hybrid_metrics.json`
- Modify: `deploy/grafana/dashboards/kairos_hybrid_metrics.json`

- [ ] **Step 1: Add Task Claim Contention Panel to Docker Provisioning**
  - Add a panel visualizing `sum(rate(ohc_task_claim_contention_total[5m])) by (mode)` to `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json`.
- [ ] **Step 2: Propagate Dashboard**
  - Copy the updated `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to `deploy/helm/ohc/dashboards/kairos_hybrid_metrics.json` and `deploy/grafana/dashboards/kairos_hybrid_metrics.json`.
- [ ] **Step 3: Verify Dashboard JSON**
  - Run `cat deploy/helm/ohc/dashboards/kairos_hybrid_metrics.json | grep -i "ohc_task_claim_contention_total"` to verify.

### Task 4: Finalize Database Metrics Dashboard

**Files:**
- Modify: `deploy/docker/grafana/provisioning/dashboards/database_metrics.json`
- Modify: `deploy/helm/ohc/dashboards/database_metrics.json` (Create if missing)
- Modify: `deploy/grafana/dashboards/database_metrics.json` (Create if missing)

- [ ] **Step 1: Apply OHC Premium Styling**
  - Ensure the HTML Text panel contains the glassmorphism CSS (like `<style>.grafana-app { backdrop-filter: blur(20px) saturate(200%) !important; background: rgba(255, 255, 255, 0.03) !important; font-family: 'Outfit', 'Inter', sans-serif !important; }</style>`) in `deploy/docker/grafana/provisioning/dashboards/database_metrics.json`.
- [ ] **Step 2: Propagate Dashboard**
  - Copy the updated `deploy/docker/grafana/provisioning/dashboards/database_metrics.json` to `deploy/helm/ohc/dashboards/database_metrics.json` and `deploy/grafana/dashboards/database_metrics.json`.
- [ ] **Step 3: Verify**
  - Run `cat deploy/helm/ohc/dashboards/database_metrics.json | grep -i "backdrop-filter"` to verify.

### Task 5: Pre-commit Steps

- [ ] **Step 1: Complete pre commit steps**
  - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

### Task 6: Submit

- [ ] **Step 1: Submit code**
  - Run `bazelisk test //...` and `bazelisk test //src/e2e:playwright_spec_coverage` to ensure all tests pass.
  - Submit the change.
