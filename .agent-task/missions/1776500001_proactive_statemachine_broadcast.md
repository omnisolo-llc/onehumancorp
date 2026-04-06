---
status: DONE
agent: jules
---

# Title: Proactive Bug Fix: StateMachine Broadcast before Transaction Commit

## Problem Statement
When using `TransitionWithTx` in the KAIROS Distributed State Machine (`statemachine` package), the state transition happens within a database transaction managed by the caller. However, the original `TransitionWithTx` returned the previous state `(string, error)` instead of a broadcast closure `(func(), error)`. This caused callers to silently fail to broadcast transitions, or worse, if a broadcast was hardcoded inside the state machine before the commit, it could broadcast uncommitted states that later rolled back.

## Research Report
According to OHC memory guidelines:
"When using the KAIROS Distributed State Machine (statemachine package), state transitions within active database queries should use TransitionWithTx or BulkTransitionWithTx to pass existing transactions and avoid deadlocks. TransitionWithTx returns a (func(), error) where the returned broadcast function MUST be called after the transaction successfully commits."

Currently, `TransitionWithTx` does not return a broadcast function, and callers in `tasks.go` like `PollTasks` and `ClaimTask` simply discard the first return value `_, err = tm.stateMachine.TransitionWithTx(...)`, meaning no broadcast happens when a task is claimed!

## Design Doc
1. Refactor `TransitionWithTx` in `srcs/server/orchestration/statemachine/machine.go` to return `(func(), error)`. The returned closure will execute `sm.broadcast` with the correct payload.
2. Update `Transition` (which delegates to `TransitionWithTx`) to call the returned closure after `tx.Commit(ctx)`.
3. Update `ClaimTask` and `PollTasks` in `srcs/server/orchestration/tasks.go` to capture the closures and execute them only after the transaction is successfully committed.

## Implementation Prompt
Dear Implementer Agent,
Please perform the refactoring described in the Design Doc. Make sure all unit tests continue to pass and `bazelisk test //srcs/server/orchestration/...` executes flawlessly.

## Priority
P0

## Estimated Scope
Small
