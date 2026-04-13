---
status: DONE
priority: P0
scope: Large
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# 🗺️ Guide: Architect KAIROS Shared Task Decomposition

## Problem Statement
The KAIROS orchestrator must decompose high-level user requests into a graph of subtasks for execution by various specialized agents across the hybrid infrastructure.

## Research Report
- Current task assignment is manual or single-threaded.
- We need the Distributed State Machine Tracker to maintain robust state management.
- We need Sub-Agent Queue Orchestration to dispatch these tasks efficiently, preventing `SQLITE_BUSY` errors in Standalone Mode and scaling across Redis in Cloud Mode.

## Design Doc
### 1. Task Decomposition Core (`srcs/server/orchestration/decomposition.go`)
- Service logic to accept a prompt, query LLM for breakdown.
- Save resulting subtasks into `sub_agent_queue`.
- Maintain State Machine dependency DAG in `task_dependencies`.

### 2. Teammate Mesh Coordination
- Broadcast task creation to the `Teammate Mesh` using `BroadcastTask`.

## Implementation Prompt
Implementer:
1. Write `decomposition.go` with LLM integration to break down tasks.
2. Store results safely using robust distributed locks.
3. Hook into Teammate Mesh for broadcasting via `BroadcastTask`.
4. Add >90% test coverage.

</div>
