---
status: PENDING
agent: Implementer
---

# Title: UltraPlan Deliberation - Deep Architectural Deliberation Cycles

## Problem Statement
KAIROS must manage "deep-deliberation cycles" (UltraPlan) for complex architectural changes (like database migrations and auth overhauls). Currently, agents do not have a structured mechanism to "deliberate" before executing. They act linearly on prompts. We need an "UltraPlan Deliberation" loop where orchestrators can pause, gather context from the Teammate Mesh, and formulate a formal plan before committing to the Shared Task List.

## Research Report
The KAIROS Orchestrator Design doc references "UltraPlan Deliberation" as the mechanism where KAIROS decomposes tasks. To implement this, we need an orchestrator engine that can transition a `shared_task` from a high-level goal into `PENDING` sub-tasks, emitting deliberation events over the Teammate Mesh so humans or other agents can observe or interrupt.

## Design Doc
1. **UltraPlan Engine:** Create `srcs/server/orchestration/ultraplan.go`. This module contains the `UltraPlanEngine`.
2. **Deliberation Loop:** Implement `Deliberate(ctx context.Context, goal string) (*DecomposedPlan, error)`. This calls the LLM (e.g. Minimax or local) to decompose the goal into an array of sub-tasks.
3. **Teammate Mesh Broadcast:** During deliberation, broadcast `mesh:ultraplan:<plan_id>` events with status updates (e.g., "Analyzing schema...", "Proposing migrations...").
4. **State Saving:** The resulting `DecomposedPlan` should be written as `PENDING` records into the `shared_tasks` table.

## Implementation Prompt
Hello Implementer agent! Please build the KAIROS UltraPlan Deliberation Engine.
1. Create `srcs/server/orchestration/ultraplan.go` and define the `UltraPlanEngine`.
2. Implement the `Deliberate` method that accepts a high-level goal, uses the `agents/local/llm.go` (or `cached_minimax_client.go`) to generate a decomposed list of tasks.
3. Broadcast status updates during this process over the Teammate Mesh using the `MeshTransport` interface.
4. Save the finalized decomposed tasks to the database via `SharedTaskListManager.CreateSharedTask`.
5. Write unit tests for the deliberation engine, mocking the LLM responses to ensure reliable task extraction.
6. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Large
