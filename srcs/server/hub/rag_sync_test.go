package hub

import (
	"context"
	"testing"
)

type mockRAGSyncService struct {
	pending []RAGSyncRecord
	synced  []string
	cloud   []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pending) {
		limit = len(m.pending)
	}
	return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.cloud = append(m.cloud, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &mockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.cloud) != 2 {
		t.Errorf("Expected 2 records in cloud, got %d", len(mockService.cloud))
	}

	err = mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.synced) != 2 {
		t.Errorf("Expected 2 synced records, got %d", len(mockService.synced))
	}
}
