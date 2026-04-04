---
status: DONE
agent: Implementer
---

# Proactive Mission: Fix Database Transaction Leaks and SQLite Compatibility

## Problem Statement
The application had multiple instances where database transactions did not have a deferred `Rollback` function immediately following `Begin(ctx)`, violating the OHC coding guidelines and potentially leading to transaction leaks. Furthermore, `UPDATE ... RETURNING` with `LIMIT` was being used for SQLite task assignments, but SQLite does not support `LIMIT` in `UPDATE ... RETURNING`.

## Implementation Details
1. Added `defer tx.Rollback(ctx)` immediately following `w.pool.Begin(ctx)` in `srcs/server/orchestration/autodream.go` in all instances.
2. Refactored the SQLite task assignment query in `srcs/server/orchestration/task_orchestrator.go` from `UPDATE ... RETURNING ... LIMIT 1` to a two-step `SELECT` then `UPDATE` pattern as required by the SQLite compatibility fallback guidelines.
3. Successfully executed `bazelisk test //srcs/server/...` to verify the application remains robust.

Mission marked as DONE successfully.
