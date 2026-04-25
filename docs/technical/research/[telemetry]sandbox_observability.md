# [telemetry] Sandbox Observability

## Problem Statement
We have limited visibility into what happens *inside* the sandbox. We need high-fidelity metrics on CPU usage, Memory consumption, Network I/O, and Syscall violations to ensure agents are operating efficiently and securely.

## Research Report
- **Competitor Analysis**: Claude Code tracks "Turn Duration" and "Cache Hits". OpenClaw monitors container health.
- **KAIROS Status**: Current telemetry tracks only `exec_total` and `violation_total` counters.

## Design Doc
- **Metric Collection**: Use `procfs` (for Standalone) or K8s metrics (for Cloud) to pull real-time resource data.
- **OTel Integration**: Add `ohc_sandbox_cpu_usage`, `ohc_sandbox_memory_bytes`, and `ohc_sandbox_network_io` Gauges.
- **Syscall Audit**: Log blocked syscalls to a dedicated security event stream in AutoDream.

## Implementation Prompt
1. Enhance `src/server/telemetry/telemetry.go` with new OTel instruments for sandbox resource tracking.
2. Update `src/server/bash_sandbox/sandbox.go` to collect metrics during execution (using `syscall.Rusage`).
3. Implement a background poller that aggregates these metrics every 5 seconds for long-running agent tasks.
4. Ensure all metrics are tagged with `agent_id`, `organization_id`, and `task_id`.
5. Create a Grafana dashboard JSON in `monitoring/grafana/dashboards/sandbox_observability.json` visualizing these metrics.

## Priority
P1

## Estimated Scope
Medium
