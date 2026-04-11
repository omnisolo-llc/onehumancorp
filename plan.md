1. **Implement Concrete RAGSyncService**
   - Implement `ragSyncService` struct implementing `RAGSyncService` interface.
   - Use `db.Provider` to fetch pending records and mark them as synced.
   - Implement `ProcessIncomingSync` with Upsert logic for Postgres.
2. **Update Database Migrations BUILD.bazel**
   - Add `032_hybrid_rag_sync.sql` to `srcs/server/db/migrations/BUILD.bazel`.
3. **Use Telemetry Metrics**
   - Use `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` inside the concrete implementation.
4. **Update Tests**
   - Write tests for the concrete implementation using an SQLite test database.
5. **Final Submission Preparation**
   - Update mission status to `DONE`.
   - Create memory and status files.
