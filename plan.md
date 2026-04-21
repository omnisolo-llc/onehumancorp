1. **Apply `perf.CoordinatorMode` in `SyncContextSync`**:
   - In `srcs/server/orchestration/sip.go`, find `SyncContextSync`.
   - After fetching the `records` from `swarm_memory_embeddings`, iterate and transform the payloads. Currently, this iteration is done sequentially: `for _, rec := range records { ... }`.
   - Update it to use `perf.CoordinatorMode(4)` for parallel payload sanitization and HTTP requests to `remoteEndpoint`.
   - To make it thread-safe, collect the `idsToDelete` via a mutex or by sending results back to a channel. Note: Since `ExecuteParallel` blocks until all finish, we can process items and aggregate safe state. Wait, HTTP requests inside `ExecuteParallel` might be concurrent and need separate `http.Client`s or reuse the same thread-safe `http.Client`.
   - Alternatively, batch the records to construct an array of processed payloads, similar to `SyncBufferedMetrics` if it does one large POST. Wait, the existing code in `SyncContextSync` does a POST *per record*: `req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(string(sanitizedPayload)))`.
   - If it does a POST per record, `ExecuteParallel` will parallelize both sanitization and network requests!
   - Use `perf.NewCoordinatorMode(4)` to parallelize it. Use a mutex for `idsToDelete` and `syncedCount`.

2. **Apply `perf.CoordinatorMode` in `SyncMissions`**:
   - In `srcs/server/orchestration/sip.go`, find `SyncMissions`.
   - Similar to above, it processes `missions` in a loop and sends a POST *per mission*.
   - Update it to use `perf.CoordinatorMode(4)`. Use a `sync.Mutex` for tracking successful synchronizations (updating `agent_missions` status in the DB transaction might not be safe, wait! The SQLite/PG transaction `tx` is not thread-safe. Wait, `tx.Exec` might not be safe to call concurrently from multiple goroutines on the same transaction).
   - In `SyncMissions`, `tx.Exec(ctx, "UPDATE agent_missions SET status = 'SYNCED' WHERE id = $1", m.id)` is done per mission. We can collect successfully synced mission IDs in a thread-safe way, and do a single batch update or individual updates *after* `ExecuteParallel` completes.
   - Wait, `SyncContextSync` collects `idsToDelete` and does a batch delete: `tx.Exec(ctx, fmt.Sprintf("DELETE FROM swarm_memory_embeddings WHERE memory_id IN (%s)", idList))`. So `SyncMissions` should collect `idsToUpdate` and do a batch update: `tx.Exec(ctx, fmt.Sprintf("UPDATE agent_missions SET status = 'SYNCED' WHERE id IN (%s)", idList))` after `ExecuteParallel`.

3. **Verify functionality**:
   - Run `bazelisk test //...` to ensure no tests are broken by these changes.

4. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
