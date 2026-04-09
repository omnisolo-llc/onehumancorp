1. **Claim the mission**: Update the mission file with specific bash commands:
   `sed -i 's/agent: Researcher/agent: Link/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   `sed -i 's/status: PENDING/status: IN_PROGRESS/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   Verify with `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5`.

2. **Database Migration**: The target table `swarm_memory_embeddings` was confirmed via `cat srcs/server/db/migrations/005_sip.sql`. Create `srcs/server/db/migrations/032_hybrid_rag_sync.sql` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/db/migrations/032_hybrid_rag_sync.sql
-- 032_hybrid_rag_sync.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
FILE_EOF
   ```
   Verify with `cat srcs/server/db/migrations/032_hybrid_rag_sync.sql`.

3. **Update Bazel DB BUILD**: Add the new migration to `srcs/server/db/BUILD.bazel` using `sed`. Based on `cat srcs/server/db/BUILD.bazel`, the last file in `embedsrcs` is `"migrations/031_agent_missions_updated_at.sql",`. Use:
   ```bash
   sed -i 's|"migrations/031_agent_missions_updated_at.sql",|"migrations/031_agent_missions_updated_at.sql",\n        "migrations/032_hybrid_rag_sync.sql",|' srcs/server/db/BUILD.bazel
   ```
   Verify with `grep 032 srcs/server/db/BUILD.bazel`.

4. **Implement Interfaces and DB Implementation**:
   Create `srcs/server/hub` directory: `mkdir -p srcs/server/hub`
   Read full mission file: `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   Create `srcs/server/hub/rag_sync.go` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/hub/rag_sync.go
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	DB db.Provider
}

func NewRAGSyncService(database db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{DB: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.DB.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		err := rows.Scan(&r.ID, &r.Context, &status, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
		_, err := s.DB.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	telemetry.RecordRAGRecordSynced(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		query := "UPDATE swarm_memory_embeddings SET context = $1, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $2"
		res, err := s.DB.Exec(ctx, query, r.Context, r.ID)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}

		affected, err := res.RowsAffected()
		if err != nil {
			return err
		}

		if affected == 0 {
			query = "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)"
			_, err = s.DB.Exec(ctx, query, r.ID, r.Context)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx)
				return err
			}
		}
	}

	telemetry.RecordRAGRecordSynced(ctx, int64(len(records)))
	return nil
}
FILE_EOF
   ```
   Verify with `cat srcs/server/hub/rag_sync.go`.

5. **Telemetry Implementation**: Modify `srcs/server/telemetry/telemetry.go` to add the requested counters.
   ```bash
   cat << 'PY_EOF' > patch_telemetry.py
import re

with open("srcs/server/telemetry/telemetry.go", "r") as f:
    c = f.read()

vars_patch = """
	ragRecordsSyncedTotal              metric.Int64Counter
	ragSyncErrorsTotal                 metric.Int64Counter
"""

c = re.sub(r'(var \(\n)', r'\1' + vars_patch, c, count=1)

init_patch = """
	ragRecordsSyncedTotal, _ = m.Int64Counter("ohc.rag_sync.records_synced_total")
	ragSyncErrorsTotal, _ = m.Int64Counter("ohc.rag_sync.errors_total")
"""

c = re.sub(r'(func InitWithMeter\(m mockableMeter\) error \{\n)', r'\1' + init_patch, c, count=1)

funcs_patch = """

// RecordRAGRecordSynced increments the global counter for RAG records successfully synced.
func RecordRAGRecordSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal == nil {
		return
	}
	ragRecordsSyncedTotal.Add(ctx, count)
}

// RecordRAGSyncError increments the counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal == nil {
		return
	}
	ragSyncErrorsTotal.Add(ctx, 1)
}
"""

c = c + funcs_patch

with open("srcs/server/telemetry/telemetry.go", "w") as f:
    f.write(c)
PY_EOF
   python3 patch_telemetry.py
   rm patch_telemetry.py
   gofmt -w srcs/server/telemetry/telemetry.go
   ```
   Verify with `grep -A 10 "RecordRAGRecordSynced" srcs/server/telemetry/telemetry.go`.

6. **Implement Unit Tests**: Create `srcs/server/hub/rag_sync_test.go` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/hub/rag_sync_test.go
package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	_, err = sqliteDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	dbProvider := db.NewSqliteProvider(sqliteDB)
	service := NewRAGSyncService(dbProvider)

	ctx := context.Background()

	_, err = dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'test context 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'test context 2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 || records[0].ID != "2" {
		t.Fatalf("expected 1 pending record with ID 2, got %v", records)
	}

	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test context 3"},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var count int
	err = sqliteDB.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records (1 marked, 1 incoming), got %d", count)
	}
}
FILE_EOF
   ```
   Verify with `cat srcs/server/hub/rag_sync_test.go`.

7. **Generate BUILD files**: Run `~/go/bin/bazelisk run //:gazelle` to generate `srcs/server/hub/BUILD.bazel`. Verify with `cat srcs/server/hub/BUILD.bazel`.

8. **Run Tests**: Execute `~/go/bin/bazelisk test //srcs/server/hub/... --test_output=errors --jobs=4 --local_test_jobs=1` to ensure tests pass.

9. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

10. **Mark Mission Done**: Update the mission file:
    `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
    Verify with `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5`.
