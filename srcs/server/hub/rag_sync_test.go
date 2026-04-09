package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct {
	Records []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	var pending []hub.RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == hub.SyncStatusPending {
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
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = hub.SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &MockRAGSyncService{}

	ctx := context.Background()

	err := svc.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
		{ID: "1", Context: "test context", Vector: []float32{0.1, 0.2}, SyncStatus: hub.SyncStatusPending},
		{ID: "2", Context: "test context 2", Vector: []float32{0.3, 0.4}, SyncStatus: hub.SyncStatusSynced},
	})

	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}

	if pending[0].ID != "1" {
		t.Fatalf("Expected pending record ID 1, got %s", pending[0].ID)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after MarkSynced failed: %v", err)
	}

	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
