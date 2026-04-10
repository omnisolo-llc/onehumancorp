1. **Create Database Migration:**
   - Run `cat << 'EOF' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
EOF`
   - Run `sed -i '/"migrations\/031_agent_missions_updated_at.sql",/a \        "migrations/032_hybrid_sync_metadata.sql",' srcs/server/db/BUILD.bazel`
   - Run `git diff srcs/server/db/BUILD.bazel` to verify.

2. **Implement Go Interface & Structs:**
   - Run `cat << 'EOF' > srcs/server/hub/rag_sync.go
package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}
EOF`
   - Run `export PATH="$PATH:$HOME/go/bin" && bazelisk run //:gazelle` to generate `BUILD.bazel` in `srcs/server/hub`.
   - Run `cat srcs/server/hub/BUILD.bazel` to verify.

3. **Write Unit Tests:**
   - Run `cat << 'EOF' > srcs/server/hub/rag_sync_test.go
package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	incomingRecords []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.pendingRecords) > limit {
		return m.pendingRecords[:limit], nil
	}
	return m.pendingRecords, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingRecords = append(m.incomingRecords, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &mockRAGSyncService{}
	err := mockService.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.syncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mockService.syncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &mockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err := mockService.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.incomingRecords) != 1 {
		t.Fatalf("expected 1 incoming record, got %d", len(mockService.incomingRecords))
	}
}
EOF`
   - Run `export PATH="$PATH:$HOME/go/bin" && bazelisk run //:gazelle` to update tests.
   - Run `cat srcs/server/hub/BUILD.bazel` to verify.

4. **Run Tests:**
   - Run `export PATH="$PATH:$HOME/go/bin" && bazelisk test //srcs/server/hub/...` to verify tests pass.

5. **Pre-commit Steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Update Mission File:**
   - Run `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to finalize the mission.
