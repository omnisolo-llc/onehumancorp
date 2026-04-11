1. **Mark Mission In Progress:**
   - Execute exactly:
     ```bash
     sed -i 's/^status: PENDING/status: IN_PROGRESS/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     sed -i 's/^agent: Researcher/agent: Jules/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
     mv .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md .agent-task/missions/2026-04-07T08-02-24Z.md
     cat .agent-task/missions/2026-04-07T08-02-24Z.md | head -n 5
     ```

2. **Create Database Migration:**
   - The highest migration is 031. Create 032.
   - Execute exactly:
     ```bash
     cat << 'EOF2' > srcs/server/db/migrations/032_hybrid_rag_sync.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
EOF2
     cat srcs/server/db/migrations/032_hybrid_rag_sync.sql
     ```

3. **Update `embedsrcs` in `srcs/server/db/BUILD.bazel`:**
   - Execute exactly:
     ```bash
     cat << 'EOF2' > update_build.py
import sys

filename = "srcs/server/db/BUILD.bazel"
with open(filename, "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if '"migrations/031_agent_missions_updated_at.sql",' in line:
        lines.insert(i+1, '        "migrations/032_hybrid_rag_sync.sql",\n')
        break

with open(filename, "w") as f:
    f.writelines(lines)
EOF2
     python3 update_build.py
     rm update_build.py
     cat srcs/server/db/BUILD.bazel | grep -A 2 031_agent_missions_updated_at.sql
     ```

4. **Add Telemetry Metrics:**
   - Execute exactly:
     ```bash
     cat << 'EOF2' > add_telemetry.py
import sys

filename = "srcs/server/telemetry/telemetry.go"
with open(filename, "r") as f:
    lines = f.readlines()

insert_code = """	RagRecordsSyncedTotal, err = m.Int64Counter(
		"ohc_rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	RagSyncErrorsTotal, err = m.Int64Counter(
		"ohc_rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		errs = append(errs, err)
	}
"""

for i, line in enumerate(lines):
    if "var errs []error" in line:
        lines.insert(i+1, insert_code)
        break

for i, line in enumerate(lines):
    if "TaskQueueLengthGauge       metric.Int64UpDownCounter" in line:
        lines.insert(i+1, "	RagRecordsSyncedTotal metric.Int64Counter\n	RagSyncErrorsTotal metric.Int64Counter\n")
        break

with open(filename, "w") as f:
    f.writelines(lines)
EOF2
     python3 add_telemetry.py
     rm add_telemetry.py
     cat srcs/server/telemetry/telemetry.go | grep -C 5 RagRecordsSyncedTotal
     ```

5. **Define Go Interfaces & Structs:**
   - Execute exactly:
     ```bash
     cat << 'EOF2' > srcs/server/hub/rag_sync.go
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

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, id := range ids {
        _, err = tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
        if telemetry.RagRecordsSyncedTotal != nil {
            telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
        }
    }
    return tx.Commit(ctx)
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.provider.IsSQLite() {
			query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = excluded.last_sync_at
			`
		}

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, now)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if telemetry.RagRecordsSyncedTotal != nil {
			telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}
EOF2
     cat srcs/server/hub/rag_sync.go
     ```

6. **Implement Unit Tests:**
   - Execute exactly:
     ```bash
     cat << 'EOF2' > srcs/server/hub/rag_sync_test.go
package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	_, err = dbConn.Exec(`CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "context1", Vector: []byte("vec1")},
	}
	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert pending directly
	_, err = dbConn.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('mem2', 'context2', 'vec2', 'pending')")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem2" {
		t.Fatalf("unexpected pending records: %+v", pending)
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending2, _ := service.FetchPendingSyncs(ctx, 10)
	if len(pending2) != 0 {
		t.Fatalf("expected 0 pending, got %d", len(pending2))
	}
}
EOF2
     cat srcs/server/hub/rag_sync_test.go
     ```

7. **Run `bazelisk run //:gazelle`:**
   - Execute exactly:
     ```bash
     ~/go/bin/bazelisk run //:gazelle -- update srcs/server/hub
     ```

8. **Update Mission Status to DONE:**
   - Execute exactly:
     ```bash
     sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z.md
     TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
     cat << 'EOF2' > ".agent-task/memory/${TIMESTAMP}.yml"
type: "memory"
content: "Implemented Hybrid MCP RAG Protocol"
EOF2
     cat ".agent-task/memory/${TIMESTAMP}.yml"
     cat << 'EOF2' > ".agent-task/status/${TIMESTAMP}.yml"
type: "status"
content: "Healthy"
EOF2
     cat ".agent-task/status/${TIMESTAMP}.yml"
     cat .agent-task/missions/2026-04-07T08-02-24Z.md | head -n 5
     ```

Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit PR:**
    - Execute `~/go/bin/bazelisk test //srcs/server/...` to verify changes.
    - Submit the PR.
