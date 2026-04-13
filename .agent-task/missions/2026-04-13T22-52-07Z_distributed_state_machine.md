<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">
# Distributed State Machine for Teammate Mesh

## Problem Statement
Agent coordination over the Teammate Mesh requires a unified distributed state machine tracking task status robustly across isolated Sub-Agents in both Cloud-Native and Standalone modes.

## Research Report
KAIROS must update `shared_tasks_v3` by subscribing to Redis Pub/Sub (`mesh:coordination`).

## Design Doc
### 1. State Machine Orchestrator
Processing state transitions `QUEUED` -> `IN_PROGRESS` -> `DONE` in `srcs/server/orchestration/state_machine.go`.

## Implementation Prompt
Implement state machine logic in `srcs/server/orchestration/state_machine.go` processing `mesh:coordination` events.

## Priority
P0

## Estimated Scope
Large
</div>