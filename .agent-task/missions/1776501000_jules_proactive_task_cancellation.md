---
title: "Proactive: Implement Shared Task Cancellation"
status: DONE
agent: Jules
priority: P1
estimated_scope: Small
---

# Problem Statement
The KAIROS Orchestration layer currently supports task states like `PENDING`, `IN_PROGRESS`, `COMPLETED`, and `FAILED` via the Distributed State Machine. However, there is no explicit way to gracefully cancel a task (e.g. transitioning it to `TERMINATED_ERROR` or `FAILED` via a dedicated API) once it has been created, especially if the task becomes irrelevant or if a user decides to abort a workflow.

# Design Doc
- **Core Component:** `srcs/server/orchestration/tasks.go`
- **Feature:** Add a `CancelTask` method to the `TaskManager`.
- **Implementation:**
  - `CancelTask(ctx context.Context, taskID string, agentID string, reason string) error`
  - Acquires the task via `tm.db.Begin(ctx)`.
  - Uses the `StateMachine` to transition the task from its current state to `FAILED` or `TERMINATED_ERROR`. We will use `statemachine.StateFailed`.
  - Emits a Teammate Mesh broadcast action `CANCEL` with the updated status.
  - Telemetry should record the transition.

# Implementation Prompt
- Implement `CancelTask` in `tasks.go`.
- Add test coverage in `tasks_test.go` or a new test file.
- Verify using `bazelisk test //srcs/server/orchestration:tasks_test`.
