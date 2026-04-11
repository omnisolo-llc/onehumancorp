1. **Claim the Mission**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to be IN_PROGRESS and assigned to Jules:
     ```bash
     sed -i 's/status: PENDING/status: IN_PROGRESS\nagent: Jules/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     sed -i '/^agent: Researcher/d' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 10
     ```

2. **Schema Migration**
   - Create a new migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` containing two `ALTER TABLE ADD COLUMN` statements: `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMPTZ NULL` to the `swarm_memory_embeddings` table. Crucially, do not use `IF NOT EXISTS` on the column addition to ensure SQLite compatibility per memory guidelines.
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.
     ```bash
     cat << 'EOF' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql
     ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
     EOF
     cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql
     sed -i '/"migrations\/031_agent_missions_updated_at.sql",/a \        "migrations/032_hybrid_sync_metadata.sql",' srcs/server/db/BUILD.bazel
     git diff srcs/server/db/BUILD.bazel
     ```

3. **Go Interface & Implementation (`srcs/server/hub/rag_sync.go`)**
   - Create `srcs/server/hub` directory and write `srcs/server/hub/rag_sync.go`:
     ```bash
     mkdir -p srcs/server/hub
     cat << 'EOF' > srcs/server/hub/rag_sync.go
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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
     }

     type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
     }

     type RAGSyncServiceImpl struct {
	provider db.Provider
     }

     func NewRAGSyncService(p db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: p}
     }

     func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var statusStr string
		var vectorBytes []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &statusStr); err != nil {
			return nil, err
		}
		rec.SyncStatus = SyncStatus(statusStr)
		records = append(records, rec)
	}
	return records, nil
     }

     func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
		if _, err := s.provider.Exec(ctx, query, id); err != nil {
			return err
		}
	}
	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
     }

     func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
     VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
     ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`
		var vectorBytes []byte
		if _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, string(rec.SyncStatus)); err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}
	return nil
     }
     EOF
     cat srcs/server/hub/rag_sync.go
     ```

4. **Metrics & Observability**
   - Use a python script to inject the new telemetry metrics into `srcs/server/telemetry/telemetry.go`.
     ```bash
     cat << 'EOF' > patch_telemetry.py
     import re

     with open("srcs/server/telemetry/telemetry.go", "r") as f:
         content = f.read()

     # Add global variables
     global_vars = """
     var (
         RAGRecordsSyncedTotal metric.Int64Counter
         RAGSyncErrorsTotal    metric.Int64Counter
     )
     """
     content = content.replace("var errs []error", "var errs []error\n" + global_vars)

     # Add initialization
     init_block = """
         RAGRecordsSyncedTotal, err = m.Int64Counter(
             "rag_records_synced_total",
             metric.WithDescription("Total RAG records synced"),
         )
         if err != nil {
             errs = append(errs, err)
         }

         RAGSyncErrorsTotal, err = m.Int64Counter(
             "rag_sync_errors_total",
             metric.WithDescription("Total RAG sync errors"),
         )
         if err != nil {
             errs = append(errs, err)
         }
     """
     content = content.replace("var errs []error\n" + global_vars, "var errs []error\n" + global_vars + init_block)

     with open("srcs/server/telemetry/telemetry.go", "w") as f:
         f.write(content)
     EOF
     python3 patch_telemetry.py
     rm patch_telemetry.py
     cat srcs/server/telemetry/telemetry.go | grep -C 10 "RAGRecordsSyncedTotal"
     ```

5. **Unit Tests**
   - Write `srcs/server/hub/rag_sync_test.go`:
     ```bash
     cat << 'EOF' > srcs/server/hub/rag_sync_test.go
     package hub

     import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
     )

     func TestRAGSyncService(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	provider := db.NewSqliteProvider(dbConn)

	// Create schema
	_, err = provider.Exec(context.Background(), `CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert pending
	_, err = provider.Exec(context.Background(), `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	// Fetch pending
	pending, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("Fetch failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "1" {
		t.Fatalf("Expected 1 pending record, got %v", pending)
	}

	// Mark synced
	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch again
	pending2, _ := service.FetchPendingSyncs(context.Background(), 10)
	if len(pending2) != 0 {
		t.Fatalf("Expected 0 pending records after sync")
	}

	// Process incoming
	rec := RAGSyncRecord{ID: "2", Context: "ctx2", SyncStatus: SyncStatusSynced}
	err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{rec})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
     }
     EOF
     cat srcs/server/hub/rag_sync_test.go
     ```
   - Update Bazel dependencies and test:
     ```bash
     ~/go/bin/bazelisk run //:gazelle -- update srcs/server/hub srcs/server/telemetry
     ~/go/bin/bazelisk test //srcs/server/hub/...
     ```

6. **Finalize State & Verification**
   - Mark the mission file as `DONE` and create memory/status files:
     ```bash
     sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 10
     cat << 'EOF' > .agent-task/memory/2026-04-07T08-02-24Z.yml
     memory:
       task_completed: "Hybrid MCP RAG Protocol (Offline-to-Cloud State Sync)"
       agent: "Jules"
       details: "Implemented database migration, Go interface, and service for RAG sync protocol."
     EOF
     cat .agent-task/memory/2026-04-07T08-02-24Z.yml
     cat << 'EOF' > .agent-task/status/2026-04-07T08-02-24Z.yml
     status: "healthy"
     agent: "Jules"
     task: "Hybrid MCP RAG Protocol"
     EOF
     cat .agent-task/status/2026-04-07T08-02-24Z.yml
     ```

   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit**
   - Submit the PR with branch `jules-hybrid-rag-sync` and proper git-agnostic commit message.
