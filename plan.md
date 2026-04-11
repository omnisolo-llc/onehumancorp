1. **Implement RAGSyncService:** Implement the `RAGSyncService` in `srcs/server/hub/rag_sync.go` using `db.Provider`. Ensure that OpenTelemetry metrics don't panic on error but handle initialization errors gracefully.
2. **Update Tests:** Update `srcs/server/hub/rag_sync_test.go` to use an actual mock `db.Provider` or similar to test the implemented logic.
3. **Run Tests:** `bazelisk test //srcs/server/hub/... //srcs/server/db/...`
4. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
