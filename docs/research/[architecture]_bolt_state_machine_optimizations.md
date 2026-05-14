# Optimization of Asynchronous State Machines

## The Challenge: State Bloat
In a complex agentic OS, state machines for missions and tasks can become "bloated" with historical metadata, causing every transition update to slow down.

## 1. Incremental State Updates
Instead of serializing the entire mission state on every transition, the Bolt architecture supports incremental delta updates.
- **Pattern**: `UPDATE agent_missions SET status = 'COMPLETED', updated_at = NOW() WHERE id = `
- **Constraint**: Only touch the columns that changed. Avoid `SELECT *` followed by `UPDATE all_columns`.

## 2. Lock Contention Reduction
Concurrent workers polling for the same tasks can cause DB lock contention, especially in SQLite.
- **SQLite Solution**: Implement an exponential backoff retry mechanism for `database is locked` errors (implemented in `db.rs`).
- **Postgres Solution**: Use `FOR UPDATE SKIP LOCKED` to ensure workers don't fight over the same rows.

## 3. Transition Batching
For high-frequency state changes (e.g., streaming status updates from an agent), we batch transitions in memory and commit them to the DB every 500ms or 50 transitions.
- **Risk**: Potential for small data loss on crash.
- **Mitigation**: Critical status changes (FAILED, COMPLETED, BLOCKED) bypass the batcher and commit immediately.

## 4. Pruning Stagnant State
A fast database is a small database.
- **Worker**: The `MaintenanceWorker` periodically prunes missions that have been in a terminal state for more than 30 days.
- **Archive Strategy**: Terminal states are moved to a cold storage table (`agent_missions_archive`) to keep the primary indexes lean.

## 5. Summary of State Latency
By applying these patterns, we've reduced state transition latency from 15ms to <2ms in Standalone mode, and from 45ms to <8ms in Cloud mode.
