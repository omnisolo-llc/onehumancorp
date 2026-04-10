package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}

	for i := range m.records {
		if idMap[m.records[i].ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
			telemetry.RecordRAGRecordSynced(ctx, 1)
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	telemetry.RecordRAGRecordSynced(ctx, len(records))
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	ctx := context.Background()
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusPending},
			{ID: "3", Context: "test 3", Vector: []float32{0.5, 0.6}, SyncStatus: SyncStatusSynced},
		},
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	ids := []string{"1"}
	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pending, _ = svc.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test 4", Vector: []float32{0.7, 0.8}, SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(svc.records) != 4 {
		t.Fatalf("expected 4 total records, got %d", len(svc.records))
	}
}
