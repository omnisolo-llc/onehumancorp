---
title: "KAIROS: Scalable Sub-Agent Orchestration Queue (Phase 4)"
status: PENDING
priority: P0
scope: Large
agent: Implementer
---

# Title: KAIROS: Scalable Sub-Agent Orchestration Queue (Phase 4)

## Problem Statement
The current task queue (`srcs/server/orchestration/queue.go`) handles basic job persistence and polling but lacks the capability to spawn and manage isolated sub-agent workers dynamically. To achieve "Absolute Autonomy", KAIROS must be able to spawn sub-agents into isolated environments (e.g., separate processes or ephemeral containers) based on demand from the Shared Task List.

## Research Report
- Existing `TaskQueue` implementations (`RedisTaskQueue`, `SQLiteTaskQueue`) are passive.
- We need an `OrchestrationWorkerPool` that watches the queue and manages the lifecycle of sub-agent processes.
- For Standalone Mode, this means spawning local processes with limited resource quotas.
- For Cloud-Native Mode, this could eventually integrate with K8s Jobs or serverless functions.
- Metric tracking for "Agent Spawn Latency" and "Concurrent Agent Count" is required for observability.

## Design Doc
1. **Worker Pool Architecture**:
   - `SubAgentManager`: A background service that polls the `TaskQueue`.
   - `SpawnAgent(task Task) (AgentInstance, error)`: Logic to initialize a new agent process with the correct `Omni-Context`.
2. **Isolation Strategy**:
   - Local: Use `os/exec` with restricted environments and `cgroups` (where available) or simple resource limits.
   - Cloud: Interface with the `api/mesh/subagent_worker.go` logic to delegate to the cloud worker swarm.
3. **State Management**:
   - Track agent health via heartbeats in `.agent-task/status/`.
   - Automatically re-queue tasks if an agent process exits unexpectedly.
4. **Observability**:
   - Expose `ohc_sub_agent_spawn_total` and `ohc_sub_agent_active_gauge` metrics.

## Implementation Prompt
Implement the `SubAgentOrchestrator` in `srcs/server/orchestration/sub_agent_orchestrator.go`.
1. The orchestrator should poll the `shared_tasks` queue using `TaskManager.ClaimTask`.
2. Upon claiming a task, it must spawn an isolated sub-agent process. For now, implement the `ProcessSpawner` that runs a child Go process or a shell script simulating the agent.
3. Pass the task context (title, payload, dependencies) to the sub-agent via environment variables or a temporary JSON file.
4. Implement a watchdog that monitors the child process and updates the task status to `FAILED` if the process exits with a non-zero code.
5. Ensure the spawner respects a `MAX_CONCURRENT_AGENTS` limit (default 4 for Standalone).
6. Integrate with `telemetry.go` to record spawn events.
7. Write integration tests in `srcs/server/orchestration/sub_agent_orchestrator_test.go` demonstrating multiple tasks being processed in parallel by separate workers.

## Priority
P0

## Estimated Scope
Large
