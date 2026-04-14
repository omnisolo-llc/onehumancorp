---
status: DONE
agent: jules
---

# Mission: Sub-Agent Orchestration & State Machine Tracking

**Title:** Sub-Agent Orchestration & State Machine Tracking
**Problem Statement:** OHC lacks a unified background queue for spawning isolated sub-agents and tracking their state machine transitions across hybrid environments.
**Research Report:**
- `srcs/server/orchestration/subagent_worker.go` and `sub_agent.go` have partial implementations.
- `sub_agent_jobs` table exists but isn't consistently used by the `DefaultSubAgentSpawner`.
- Real-time tracking of sub-agent progress in the UI requires structured events.
**Design Doc:**
- **Queue Worker:** Implement a worker that polls `sub_agent_jobs` using `SKIP LOCKED` (Postgres) and Mutexes (SQLite).
- **Isolation:** For Standalone mode, use goroutines with OS-level resource limits (where possible). For Cloud, use a sidecar pattern or K8s Jobs.
- **State Machine Integration:** Every job transition (PENDING -> RUNNING -> COMPLETED/FAILED) must trigger a `MeshEvent` and update `shared_tasks`.
- **Telemetry:** Record `ohc_sub_agent_execution_duration_seconds` and `ohc_sub_agent_failures_total`.
**Implementation Prompt:**
- Update `srcs/server/orchestration/subagent_worker.go` to poll the `sub_agent_jobs` table.
- Link the worker to `TaskStateMachine` to ensure atomic status updates.
- Implement `SpawnIsolated(ctx context.Context, job *Job)` in `SubAgentSpawner`.
- Ensure the `DefaultSubAgentSpawner` writes heartbeats to `.agent-task/status/{timestamp}.yml` as required by OHC-SIP.
- Write a test in `srcs/server/orchestration/sub_agent_test.go` verifying that a failed job correctly transitions the parent task to FAILED.
**Priority:** P2
**Estimated Scope:** Medium
