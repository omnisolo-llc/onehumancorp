1. **Create Database Migration:**
   - Execute bash command:
     ```bash
     cat << 'EOF' > srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql
     -- +goose Up
     -- Add hybrid sync metadata to agent_memories
     -- SQLite compatible separate ADD COLUMN statements
     ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP;

     -- +goose Down
     -- ALTER TABLE agent_memories DROP COLUMN sync_status;
     -- ALTER TABLE agent_memories DROP COLUMN last_sync_at;
     EOF
     ```
   - Verify: `cat srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql`

2. **Implement RAGSyncService:**
   - Execute bash command:
     ```bash
     cat << 'EOF' > srcs/server/hub/rag_sync.go
     package hub

     import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
     }

     type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
     }

     var (
	meter              = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedTotal metric.Int64Counter
	syncErrorsTotal    metric.Int64Counter
     )

     func init() {
	var err error
	recordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	if err != nil {
		panic(fmt.Errorf("failed to create recordsSyncedTotal metric: %w", err))
	}
	syncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	if err != nil {
		panic(fmt.Errorf("failed to create syncErrorsTotal metric: %w", err))
	}
     }

     type sqlRAGSyncService struct {
	dbWrapper *db.DB
     }

     func NewSQLRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &sqlRAGSyncService{dbWrapper: dbWrapper}
     }

     func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch")))
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var embeddingJSON string
		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingJSON, &rec.SyncStatus, &lastSyncAt); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_scan")))
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		if embeddingJSON != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingJSON), &vec); err != nil {
				syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_unmarshal")))
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
			rec.Vector = vec
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_rows_err")))
		return nil, fmt.Errorf("row error: %w", err)
	}
	return records, nil
     }

     func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_begin")))
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback()
	}()

	query := `UPDATE agent_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
	now := time.Now().UTC()
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_exec")))
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_commit")))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	recordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
     }

     func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_begin")))
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback()
	}()

	query := `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5, $6)
		ON CONFLICT (id) DO UPDATE SET
		content = excluded.content,
		embedding = excluded.embedding,
		sync_status = excluded.sync_status,
		last_sync_at = excluded.last_sync_at`

	for _, rec := range records {
		var embeddingJSON string
		if rec.Vector != nil {
			b, err := json.Marshal(rec.Vector)
			if err != nil {
				syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_marshal")))
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			embeddingJSON = string(b)
		}

		var lastSyncAt sql.NullTime
		if !rec.LastSyncAt.IsZero() {
			lastSyncAt.Time = rec.LastSyncAt
			lastSyncAt.Valid = true
		}

		// Assume incoming records have 'synced' status
		status := SyncStatusSynced
		if rec.SyncStatus != "" {
			status = rec.SyncStatus
		}
		orgID := "default"

		if _, err := tx.Exec(ctx, query, rec.ID, orgID, rec.Context, embeddingJSON, string(status), lastSyncAt); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_exec")))
			return fmt.Errorf("failed to upsert record id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_commit")))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
     }
     EOF
     ```
   - Verify: `cat srcs/server/hub/rag_sync.go`

3. **Generate BUILD.bazel for Hub:**
   - Execute bash command: `export PATH="$PATH:$HOME/go/bin" && bazelisk run //:gazelle`
   - Verify: `cat srcs/server/hub/BUILD.bazel`

4. **Add Tests:**
   - Execute bash command:
     ```bash
     cat << 'EOF' > srcs/server/hub/rag_sync_test.go
     package hub

     import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
     )

     func TestRAGSyncService(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	// Manually run migration-like setup since memory db is fresh
	_, err = dbWrapper.Exec(ctx, `CREATE TABLE agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status TEXT DEFAULT 'pending',
		last_sync_at TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewSQLRAGSyncService(dbWrapper)

	// 1. ProcessIncomingSync
	recordsToIncoming := []RAGSyncRecord{
		{ID: "mem1", Context: "Context 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
		{ID: "mem2", Context: "Context 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusPending},
	}
	err = service.ProcessIncomingSync(ctx, recordsToIncoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Verify contents
	if pending[0].Context != "Context 1" && pending[1].Context != "Context 1" {
		t.Errorf("missing Context 1")
	}
	if len(pending[0].Vector) != 2 {
		t.Errorf("missing Vector")
	}

	// 3. MarkSynced
	err = service.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Re-fetch pending, should be 1
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Fatalf("expected 1 pending record after MarkSynced, got %d", len(pendingAfter))
	}
	if pendingAfter[0].ID != "mem2" {
		t.Errorf("expected mem2 to still be pending")
	}
     }
     EOF
     ```
   - Verify: `cat srcs/server/hub/rag_sync_test.go`
   - Execute bash command: `export PATH="$PATH:$HOME/go/bin" && bazelisk run //:gazelle`

5. **Update Mission and Status:**
   - Execute bash command:
     ```bash
     cat << 'EOF' > .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     ---
     status: DONE
     agent: Implementer
     ---
     # Title: Hybrid MCP RAG Protocol: Bridging Standalone SQLite to Cloud PostgreSQL

     ## Problem Statement
     ...
     EOF
     ```
   - Execute bash command:
     ```bash
     export TS=$(date -Iseconds -u | sed 's/:/-/g')
     cat << EOF > .agent-task/status/${TS}.yml
     agent: Implementer
     status: healthy
     last_action: Implemented Hybrid MCP RAG Protocol sync service and schema.
     timestamp: ${TS}
     EOF
     ```

6. **Run Tests:**
   - Execute bash command: `export PATH="$PATH:$HOME/go/bin" && bazelisk test //srcs/server/hub/...`

7. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
