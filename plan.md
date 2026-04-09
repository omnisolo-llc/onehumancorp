1. **Claim the mission**: Update the mission file with specific bash commands:
   `sed -i 's/agent: Researcher/agent: Link/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   `sed -i 's/status: PENDING/status: IN_PROGRESS/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   Verify with `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5`.

2. **Database Migration**: Create `srcs/server/db/migrations/032_hybrid_rag_sync.sql` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/db/migrations/032_hybrid_rag_sync.sql
-- 032_hybrid_rag_sync.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
FILE_EOF
   ```
   Verify with `cat srcs/server/db/migrations/032_hybrid_rag_sync.sql`.

3. **Update Bazel DB BUILD**: Add the new migration to `srcs/server/db/BUILD.bazel`. Use Python to reliably insert it since `sed` can be error-prone with multiline lists.
   ```bash
   cat << 'PY_EOF' > patch.py
import re
with open("srcs/server/db/BUILD.bazel", "r") as f:
    c = f.read()
c = re.sub(r'("migrations/031_agent_missions_updated_at\.sql",)', r'\1\n        "migrations/032_hybrid_rag_sync.sql",', c, count=1)
with open("srcs/server/db/BUILD.bazel", "w") as f:
    f.write(c)
PY_EOF
   python3 patch.py
   rm patch.py
   ```
   Verify with `grep 032 srcs/server/db/BUILD.bazel`.

4. **Implement Interfaces & Telemetry**:
   Create `srcs/server/hub` directory: `mkdir -p srcs/server/hub`
   Create `srcs/server/hub/rag_sync.go` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/hub/rag_sync.go
package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel/metric"
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
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncTelemetry(m metric.Meter) error {
	var err error
	ragRecordsSyncedTotal, err = m.Int64Counter(
		"ohc.rag_sync.records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsTotal, err = m.Int64Counter(
		"ohc.rag_sync.errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	return err
}

func RecordRAGRecordSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}
FILE_EOF
   ```
   Verify with `cat srcs/server/hub/rag_sync.go`.

5. **Implement Unit Tests**: Create `srcs/server/hub/rag_sync_test.go` using:
   ```bash
   cat << 'FILE_EOF' > srcs/server/hub/rag_sync_test.go
package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	IncomingRecords []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingRecords = append(m.IncomingRecords, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	err = mock.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.SyncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mock.SyncedIDs))
	}

	err = mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.IncomingRecords) != 2 {
		t.Fatalf("expected 2 incoming records, got %d", len(mock.IncomingRecords))
	}
}
FILE_EOF
   ```
   Verify with `cat srcs/server/hub/rag_sync_test.go`.

6. **Generate BUILD files**: Run `~/go/bin/bazelisk run //:gazelle` to generate `srcs/server/hub/BUILD.bazel`. Verify with `cat srcs/server/hub/BUILD.bazel`.

7. **Run Tests**: Execute `~/go/bin/bazelisk test //srcs/server/hub/... --test_output=errors --jobs=4 --local_test_jobs=1` to ensure tests pass.

8. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

9. **Mark Mission Done**: Update the mission file:
   `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   Verify with `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5`.
