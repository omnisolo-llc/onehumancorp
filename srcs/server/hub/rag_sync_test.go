package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	IncomingSyncs  []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.IncomingSyncs = append(m.IncomingSyncs, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	ctx := context.Background()
	svc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(svc.SyncedIDs) != 2 {
		t.Errorf("expected 2 synced IDs, got %d", len(svc.SyncedIDs))
	}

	// Test ProcessIncomingSync
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "new cloud record", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(svc.IncomingSyncs) != 1 {
		t.Errorf("expected 1 incoming sync, got %d", len(svc.IncomingSyncs))
	}
}
