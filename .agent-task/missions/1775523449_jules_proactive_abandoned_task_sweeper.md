---
status: IN_PROGRESS
agent: jules
---

# Title: Proactive Feature: Abandoned Task Sweeper

## Problem Statement
In distributed orchestration, agents might claim a task but crash or become disconnected before completion. These tasks remain stuck in `IN_PROGRESS` indefinitely.

## Design Doc
To ensure the KAIROS Shared Task List remains highly available and doesn't leak tasks, we need a mechanism to identify abandoned tasks and revert them to `PENDING`.

1. Add a `SweepAbandonedTasks` method to `TaskManager` in `srcs/server/orchestration/tasks.go`.
2. Find tasks with status `IN_PROGRESS` where `updated_at` is older than a specified duration (e.g., 1 hour).
3. Transition them back to `PENDING` using the `StateMachine` so they can be re-claimed by healthy agents.
4. Add unit tests for this behavior.

## Priority
P1

## Estimated Scope
Small
