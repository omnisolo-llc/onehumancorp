1. **Mark Mission IN_PROGRESS**: Use `run_in_bash_session` to execute `sed -i 's/status: PENDING/status: IN_PROGRESS\nagent: Jules/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to update the mission file.
2. **Verify Mission IN_PROGRESS**: Use `run_in_bash_session` to execute `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to ensure the status is updated.
3. **Database Migration**: Use `run_in_bash_session` to execute:
```bash
cat << 'EOF2' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql
-- 032_hybrid_sync_metadata.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
EOF2
```
4. **Verify Database Migration**: Use `run_in_bash_session` to execute `cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql` to verify it was created.
5. **Go Interface Definition**: Use `run_in_bash_session` to execute:
```bash
cat << 'EOF2' > srcs/server/hub/rag_sync.go
package hub

import (
	"context"
	"time"
	"log/slog"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         metric.Meter
	RecordsSynced metric.Int64Counter
	SyncErrors    metric.Int64Counter
)

func init() {
	var err error
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		slog.Error("Failed to initialize RecordsSynced metric", "error", err)
	}
	SyncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		slog.Error("Failed to initialize SyncErrors metric", "error", err)
	}
}

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
EOF2
```
6. **Verify Go Interface Definition**: Use `run_in_bash_session` to execute `cat srcs/server/hub/rag_sync.go` to verify the code.
7. **Mock implementation & Test**: Use `run_in_bash_session` to execute:
```bash
cat << 'EOF2' > srcs/server/hub/rag_sync_test.go
package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.PendingRecords {
		for _, id := range ids {
			if m.PendingRecords[i].ID == id {
				m.PendingRecords[i].SyncStatus = SyncStatusSynced
				m.PendingRecords[i].LastSyncAt = time.Now()
			}
		}
	}
	RecordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockService.PendingRecords[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}
}
EOF2
```
8. **Verify Mock implementation & Test**: Use `run_in_bash_session` to execute `cat srcs/server/hub/rag_sync_test.go` to verify the code.
9. **Add to Bazel BUILD**: Use `run_in_bash_session` to execute:
```bash
cat << 'EOF2' > srcs/server/hub/BUILD.bazel
load("@rules_go//go:def.bzl", "go_library", "go_test")

go_library(
    name = "hub",
    srcs = ["rag_sync.go"],
    importpath = "github.com/onehumancorp/mono/srcs/server/hub",
    visibility = ["//visibility:public"],
    deps = [
        "@io_opentelemetry_go_otel//:otel",
        "@io_opentelemetry_go_otel_metric//:metric",
    ],
)

go_test(
    name = "hub_test",
    srcs = ["rag_sync_test.go"],
    embed = [":hub"],
)
EOF2
```
10. **Verify Bazel BUILD**: Use `run_in_bash_session` to execute `cat srcs/server/hub/BUILD.bazel` to verify the file.
11. **Mark Mission DONE**: Use `run_in_bash_session` to execute `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to mark it done.
12. **Verify Mission DONE**: Use `run_in_bash_session` to execute `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to ensure the status is updated.
13. **Create Observability Heartbeat**: Use `run_in_bash_session` to execute `mkdir -p .agent-task/status && cat << 'EOF2' > .agent-task/status/$(date -u +%Y-%m-%dT%H-%M-%SZ).yml
agent: Jules
status: healthy
last_heartbeat: $(date -u +%Y-%m-%dT%H:%M:%SZ)
EOF2`
14. **Verify Observability Heartbeat**: Use `run_in_bash_session` to execute `ls -l .agent-task/status/` and `cat .agent-task/status/*.yml` to verify it was created.
15. **Verification & Tests**: Run `bazelisk test //srcs/server/hub/...` to verify tests pass.
16. **Pre-commit Instructions**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
17. **Submit PR**: Commit and submit.
