package hub

import (
	"context"
	"errors"
	"testing"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	incomingCalled bool
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pendingRecords) {
		return m.pendingRecords, nil
	}
	return m.pendingRecords[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingCalled = true
	if len(records) == 0 {
		return errors.New("no records provided")
	}
	return nil
}

func TestRAGSyncService_Interface(t *testing.T) {
	var service RAGSyncService
	mock := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}
	service = mock

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mock.syncedIDs) != 2 {
		t.Errorf("expected 2 synced IDs, got %d", len(mock.syncedIDs))
	}

	// Test ProcessIncomingSync
	err = service.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if !mock.incomingCalled {
		t.Error("expected ProcessIncomingSync to be called")
	}

	// Test error case
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err == nil {
		t.Error("expected error for empty records in ProcessIncomingSync")
	}
}
