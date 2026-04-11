1. **Implement RAGSyncService:** Create `srcs/server/hub/rag_sync_impl.go` which provides a concrete production implementation (`ragSyncServiceImpl`) of `RAGSyncService` using `db.Provider`. Ensure it correctly increments the OpenTelemetry metrics on success and error.
2. **Write Concrete Tests:** Update `srcs/server/hub/rag_sync_test.go` to test the actual implementation using a mock `db.Provider` or in-memory SQLite (`db.NewSqliteProvider`).
3. **Run tests:** Run `bazelisk run //:gazelle` and `bazelisk test //srcs/server/hub/...` to ensure tests pass.
4. **Complete pre-commit steps:** Request another code review to ensure the implementation is sufficient and initiate memory recording.
5. **Submit:** Submit the changes.
