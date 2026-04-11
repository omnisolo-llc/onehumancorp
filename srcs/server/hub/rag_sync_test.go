package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	records []RAGSyncRecord
	syncedIDs []string
	processErr error
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
	m.syncedIDs = append(m.syncedIDs, ids...)
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.processErr != nil {
		return m.processErr
	}
	m.records = append(m.records, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
			{ID: "3", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(svc.syncedIDs) != 1 || svc.syncedIDs[0] != "1" {
		t.Errorf("expected synced ID '1', got %v", svc.syncedIDs)
	}

	pending, _ = svc.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 || pending[0].ID != "2" {
		t.Errorf("expected 1 pending record with ID '2', got %v", pending)
	}

	newRecords := []RAGSyncRecord{{ID: "4", SyncStatus: SyncStatusPending}}
	err = svc.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = svc.FetchPendingSyncs(ctx, 10)
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records after processing, got %d", len(pending))
	}

	svc.processErr = errors.New("processing failed")
	err = svc.ProcessIncomingSync(ctx, newRecords)
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}
