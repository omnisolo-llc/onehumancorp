package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// MockRAGSyncService is a mock implementation of RAGSyncService.
type MockRAGSyncService struct {
	Records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
			telemetry.RecordRAGRecordsSynced(ctx, 1)
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	telemetry.RecordRAGRecordsSynced(ctx, len(records))
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	ctx := context.Background()
	service := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "test context", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
		},
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if service.Records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected record to be synced, got %s", service.Records[0].SyncStatus)
	}

	// Trigger error telemetry conceptually
	telemetry.RecordRAGSyncError(ctx)
}
