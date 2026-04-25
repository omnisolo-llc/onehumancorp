1. **Refactor `src/server/memory/vector_repository.go`**
   - Use `run_in_bash_session` to execute a Python script to update the `PruneStale` method signature to `PruneStale(ctx context.Context, organizationID string, olderThan time.Time) error` and update the underlying SQL query to include `organization_id = $1` and `created_at < $2`.
   - Add a new method `GetRecentMemories(ctx context.Context, organizationID string, since time.Time) ([]*EmbeddingRecord, error)` to fetch records that need conflict resolution. It will execute `SELECT id, organization_id, COALESCE(agent_id, ''), content, embedding, source_type, created_at FROM consolidated_memory WHERE organization_id = $1 AND created_at >= $2 ORDER BY created_at DESC LIMIT 100`.
2. **Verify changes to `src/server/memory/vector_repository.go`**
   - Use `cat` or `read_file` to verify the updates.
3. **Refactor `src/server/memory/autodream/service.go`**
   - Use `run_in_bash_session` to execute a Python script to change `PruneStaleContext` signature to `PruneStaleContext(ctx context.Context, orgID string, maxAge time.Duration) error` and update its implementation to call `s.vectorRepo.PruneStale(ctx, orgID, time.Now().Add(-maxAge))`.
   - Remove `StartBackgroundPruner`.
   - Add `ResolveConflicts(ctx context.Context, orgID string) error` to fetch recent memories via `GetRecentMemories(ctx, orgID, time.Now().Add(-24*time.Hour))`, iterate through them, generate embeddings, search for similar older ones via `s.vectorRepo.SemanticSearch`, prompt the LLM to merge them, upsert the resolved memory with ID `{originalID}-merged`, and delete the older ones.
4. **Verify changes to `src/server/memory/autodream/service.go`**
   - Use `cat` or `read_file` to verify the updates.
5. **Update tests in `src/server/memory/vector_repository_test.go`**
   - Use `run_in_bash_session` to update `repo.PruneStale(ctx, oldTime)` to `repo.PruneStale(ctx, "org-1", oldTime)` in `src/server/memory/vector_repository_test.go`.
6. **Verify changes to tests**
   - Use `cat` to read the tests and ensure they are correct.
7. **Test all changes**
   - Use `run_in_bash_session` to execute `bazelisk test //...` to verify all tests pass.
8. **Complete pre-commit steps**
   - Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
9. **Submit PR**
   - Complete the task by opening a pull request with the title "⚙️ Consolidator: persistent memory layer, conflict resolution, and pruning" and describe the architecture and features implemented.
