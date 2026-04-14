---
status: DONE
agent: Miser
---

# Title: Proactive Implement SQLite Vacuuming and Aggressive Pruning

## Problem Statement
In Standalone Desktop Mode, local SQLite databases can grow unboundedly due to continuous telemetry buffering, ephemeral logging, and cache storage. Although row deletion functions exist (e.g., pruning old caches or deleting synced telemetry), SQLite does not automatically reclaim disk space unless \`VACUUM\` is run. This unbounded storage footprint directly violates the Miser Cost Engineer mandate of optimizing host machine efficiency and lowering resource consumption.

## Research Report
- SQLite requires manual \`VACUUM\` or \`PRAGMA auto_vacuum = FULL\` to shrink file sizes.
- Relying on auto_vacuum increases fragmentation. A periodic manual \`VACUUM\` is much more efficient.
- Existing telemetry SyncWorker deletes records, but file size only grows.
- We must add an automated periodic \`VACUUM\` routine and an aggressive prune operation for old ephemeral data (like failed telemetry) in Standalone Mode.

## Design Doc
1. **Database Routine**:
   - Implement a new worker or routine \`StartStorageOptimizerDaemon\` in \`srcs/server/db/\` or \`telemetry/\`.
   - Run \`db.Exec("VACUUM")\` periodically (e.g., every 24 hours, but for testing or manual triggers, expose an internal or interval parameter).
   - Before \`VACUUM\`, execute cleanup: \`DELETE FROM telemetry_buffer WHERE created_at < NOW() - INTERVAL '7 days'\` (translate to SQLite datetime syntax: \`datetime('now', '-7 day')\`).

## Implementation Prompt
Hello Implementer agent!
1. Add a method to \`SIPDB\` or \`db.Provider\` to execute database maintenance/vacuum.
2. In \`srcs/server/telemetry/sync_worker.go\` (or a similar location), start a periodic goroutine that performs aggressive garbage collection on \`telemetry_buffer\` and runs \`VACUUM\` if the provider is SQLite.
3. Ensure no regressions occur in cloud mode (Postgres).

## Priority
P2

## Estimated Scope
Small
