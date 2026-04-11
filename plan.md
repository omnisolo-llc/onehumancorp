1. **Create the DB Migration:**
   - I will explicitly run:
     ```bash
     cat << 'INNEREOF' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql
     ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status TEXT DEFAULT 'pending';
     ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
     INNEREOF
     ```
   - Then update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs` using `sed`:
     ```bash
     sed -i 's/"migrations\/031_agent_missions_updated_at.sql",/"migrations\/031_agent_missions_updated_at.sql",\n        "migrations\/032_hybrid_sync_metadata.sql",/g' srcs/server/db/BUILD.bazel
     ```
   - I will verify with `cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql` and `cat srcs/server/db/BUILD.bazel | grep 032`.

2. **Add Telemetry Definitions:**
   - I will inject telemetry metrics directly into `srcs/server/telemetry/telemetry.go` using a python script:
     ```python
     cat << 'INNEREOF' > patch_telemetry.py
     with open("srcs/server/telemetry/telemetry.go", "r") as f:
         content = f.read()

     # Add variables
     content = content.replace("cacheMissesCounter         metric.Int64Counter", "cacheMissesCounter         metric.Int64Counter\n\tRagRecordsSyncedTotal metric.Int64Counter\n\tRagSyncErrorsTotal metric.Int64Counter")

     # Add initialization
     init_block = """	cacheMissesCounter, err = m.Int64Counter(
		"ohc_cache_misses_total",
		metric.WithDescription("Total cache misses for LLM operations"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	RagRecordsSyncedTotal, err = m.Int64Counter("rag_records_synced_total")
	if err != nil { errs = append(errs, err) }

	RagSyncErrorsTotal, err = m.Int64Counter("rag_sync_errors_total")
	if err != nil { errs = append(errs, err) }"""

     content = content.replace('	cacheMissesCounter, err = m.Int64Counter(\n\t\t"ohc_cache_misses_total",\n\t\tmetric.WithDescription("Total cache misses for LLM operations"),\n\t)\n\tif err != nil {\n\t\terrs = append(errs, err)\n\t}', init_block)

     helpers = """
     // RecordRagRecordsSynced increments the synced records counter
     func RecordRagRecordsSynced(ctx context.Context, count int64) {
         if RagRecordsSyncedTotal != nil {
             RagRecordsSyncedTotal.Add(ctx, count)
         }
     }

     // RecordRagSyncError increments the sync error counter
     func RecordRagSyncError(ctx context.Context) {
         if RagSyncErrorsTotal != nil {
             RagSyncErrorsTotal.Add(ctx, 1)
         }
     }
     """
     content = content + helpers

     with open("srcs/server/telemetry/telemetry.go", "w") as f:
         f.write(content)
     INNEREOF
     python3 patch_telemetry.py
     ```
   - I will verify the changes using `cat srcs/server/telemetry/telemetry.go | grep -A 5 RagRecordsSyncedTotal`.

3. **Implement concrete logic (Go Interface and Struct):**
   - I will run the exact following bash command:
     ```bash
     cat << 'INNEREOF' > srcs/server/hub/rag_sync.go
     package hub

     import (
         "context"
         "time"

         "github.com/onehumancorp/mono/srcs/server/db"
         "github.com/onehumancorp/mono/srcs/server/telemetry"
     )

     type SyncStatus string

     const (
         SyncStatusPending SyncStatus = "pending"
         SyncStatusSynced  SyncStatus = "synced"
         SyncStatusError   SyncStatus = "error"
     )

     type RAGSyncRecord struct {
         ID         string
         Context    string
         Vector     []byte
         SyncStatus SyncStatus
         LastSyncAt time.Time
     }

     type RAGSyncService interface {
         FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
         MarkSynced(ctx context.Context, ids []string) error
         ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
     }

     type ragSyncService struct {
         provider db.Provider
     }

     func NewRAGSyncService(provider db.Provider) RAGSyncService {
         return &ragSyncService{provider: provider}
     }

     func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
         query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
         rows, err := s.provider.Query(ctx, query, limit)
         if err != nil {
             telemetry.RecordRagSyncError(ctx)
             return nil, err
         }
         defer rows.Close()

         var records []RAGSyncRecord
         for rows.Next() {
             var rec RAGSyncRecord
             var lastSync *time.Time
             var status *string
             if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &status, &lastSync); err != nil {
                 telemetry.RecordRagSyncError(ctx)
                 continue
             }
             if status != nil {
                 rec.SyncStatus = SyncStatus(*status)
             } else {
                 rec.SyncStatus = SyncStatusPending
             }
             if lastSync != nil {
                 rec.LastSyncAt = *lastSync
             }
             records = append(records, rec)
         }
         return records, nil
     }

     func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
         if len(ids) == 0 {
             return nil
         }
         now := time.Now()
         for _, id := range ids {
             _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
             if err != nil {
                 telemetry.RecordRagSyncError(ctx)
                 return err
             }
         }
         telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
         return nil
     }

     func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
         if len(records) == 0 {
             return nil
         }
         now := time.Now()
         for _, rec := range records {
             var exists bool
             err := s.provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1)", rec.ID).Scan(&exists)
             if err != nil {
                 telemetry.RecordRagSyncError(ctx)
                 return err
             }
             if exists {
                 _, err = s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = $3 WHERE memory_id = $4", rec.Context, rec.Vector, now, rec.ID)
             } else {
                 _, err = s.provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)", rec.ID, rec.Context, rec.Vector, now)
             }
             if err != nil {
                 telemetry.RecordRagSyncError(ctx)
                 return err
             }
         }
         telemetry.RecordRagRecordsSynced(ctx, int64(len(records)))
         return nil
     }
     INNEREOF
     ```
   - I will run `~/go/bin/bazelisk run //:gazelle -- update srcs/server/hub` to update BUILD rules.
   - I will verify the file creation using `cat srcs/server/hub/rag_sync.go` and `cat srcs/server/hub/BUILD.bazel`.

4. **Write Tests:**
   - I will run the exact following bash command:
     ```bash
     cat << 'INNEREOF' > srcs/server/hub/rag_sync_test.go
     package hub

     import (
         "context"
         "testing"

         _ "modernc.org/sqlite"
         "github.com/onehumancorp/mono/srcs/server/db"
     )

     func TestRAGSyncService(t *testing.T) {
         provider, err := db.NewSqliteProvider("file::memory:?cache=shared")
         if err != nil {
             t.Fatalf("Failed to create provider: %v", err)
         }
         defer provider.Close()

         _, err = provider.Exec(context.Background(), `CREATE TABLE swarm_memory_embeddings (
             memory_id TEXT PRIMARY KEY,
             context TEXT NOT NULL,
             vector_embedding BLOB,
             sync_status TEXT DEFAULT 'pending',
             last_sync_at TIMESTAMPTZ NULL
         )`)
         if err != nil {
             t.Fatalf("Failed to create table: %v", err)
         }

         svc := NewRAGSyncService(provider)
         ctx := context.Background()

         _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'ctx1', X'00', 'pending')")
         if err != nil {
             t.Fatalf("Failed to insert: %v", err)
         }

         pending, err := svc.FetchPendingSyncs(ctx, 10)
         if err != nil {
             t.Fatalf("Failed to fetch pending: %v", err)
         }
         if len(pending) != 1 || pending[0].ID != "1" {
             t.Errorf("Expected 1 pending record with ID 1, got %v", pending)
         }

         err = svc.MarkSynced(ctx, []string{"1"})
         if err != nil {
             t.Fatalf("Failed to mark synced: %v", err)
         }

         pending, err = svc.FetchPendingSyncs(ctx, 10)
         if err != nil {
             t.Fatalf("Failed to fetch pending again: %v", err)
         }
         if len(pending) != 0 {
             t.Errorf("Expected 0 pending records, got %v", pending)
         }

         err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "2", Context: "ctx2", Vector: []byte{0x00}, SyncStatus: SyncStatusPending}})
         if err != nil {
             t.Fatalf("Failed to process incoming: %v", err)
         }
         var exists bool
         err = provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = '2')").Scan(&exists)
         if err != nil || !exists {
             t.Fatalf("Expected incoming record to be inserted")
         }
     }
     INNEREOF
     ```
   - Update `srcs/server/hub/BUILD.bazel` to include testing dependencies properly via Gazelle update.
   - Verify test file existence with `ls -la srcs/server/hub/`.

5. **Final Testing:**
   - I will run `~/go/bin/bazelisk test //srcs/server/hub/... //srcs/server/db/...` to ensure tests pass.

6. **Update Mission Status and Observability:**
   - Run exact commands:
     ```bash
     mv .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md .agent-task/missions/2026-04-07T08-02-24Z.md
     sed -i 's/status: PENDING/status: DONE\nagent: Jules/' .agent-task/missions/2026-04-07T08-02-24Z.md
     cat << 'INNEREOF' > .agent-task/memory/20260411T120000Z.yml
     type: implementation_report
     mission_id: 2026-04-07T08-02-24Z
     summary: Implemented Hybrid MCP RAG sync interfaces and metadata schema.
     INNEREOF
     cat << 'INNEREOF' > .agent-task/status/20260411T120000Z.yml
     status: HEALTHY
     agent: Jules
     mission: 2026-04-07T08-02-24Z
     INNEREOF
     ```
   - Verify file creation and contents using:
     ```bash
     cat .agent-task/missions/2026-04-07T08-02-24Z.md | head -n 5
     cat .agent-task/memory/20260411T120000Z.yml
     cat .agent-task/status/20260411T120000Z.yml
     ls -la .agent-task/status/20260411T120000Z.yml
     ls -la .agent-task/memory/20260411T120000Z.yml
     ```

Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit Changes:**
   - Submit via `submit` using an appropriate PR format (e.g. `agent: Jules`).
