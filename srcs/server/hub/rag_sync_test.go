package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(svc.Processed) != 2 {
		t.Fatalf("expected 2 processed records, got %d", len(svc.Processed))
	}

	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(svc.MarkedSynced) != 2 {
		t.Fatalf("expected 2 marked records, got %d", len(svc.MarkedSynced))
	}

	RecordSyncSuccess(ctx, 2)
	RecordSyncError(ctx, 1)
}
