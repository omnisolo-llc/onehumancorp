---
status: PENDING
priority: P0
scope: Medium
title: "KAIROS: Manage UltraPlan Deliberation Cycles"
---

# Title: Manage UltraPlan Deliberation Cycles

## Problem Statement
The OHC KAIROS Orchestrator needs to manage deep-deliberation cycles (UltraPlan) for complex architectural changes, such as database migrations and Auth overhauls. This enables agents to formulate comprehensive step-by-step master plans before attempting execution, reducing cascading failures.

## Research Report
- Complex tasks cannot be executed in a single shot.
- We need a deliberation phase where an agent plans, peer-reviews, and refines.
- The outcome is an "UltraPlan" document or sequence.

## Design Doc
1.  **UltraPlan Orchestrator:**
    - A service in `srcs/server/orchestration/ultraplan.go`.
    - Takes a raw, complex prompt and initiates a deliberation cycle.
    - Interfaces with the LLM multiple times (propose, critique, refine).
2.  **Storage:**
    - The finalized UltraPlan should be stored in the database or filesystem (`.agent-task/ultraplans/`) for execution by Sub-Agents.

## Implementation Prompt
- Implement the `UltraPlanDeliberator` logic in `srcs/server/orchestration/ultraplan.go`.
- Define the multi-step LLM chain (Propose -> Critique -> Refine).
- Define the output format (structured JSON or markdown) and store it robustly using the `DistributedStateMachine` for tracking progress.
- Write tests to ensure >90% code coverage.
