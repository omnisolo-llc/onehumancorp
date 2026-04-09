package hub

import (
	"context"
	"errors"
	"testing"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	markErr        error
	processErr     error
	markedIDs      []string
	processedRecs  []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.pendingRecords) {
		return m.pendingRecords[:limit], nil
	}
	return m.pendingRecords, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.markedIDs = ids
	return m.markErr
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.processedRecs = records
	return m.processErr
}

func TestFetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	mockSvc := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	recs, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(recs) != 2 {
		t.Errorf("expected 2 records, got %d", len(recs))
	}
}

func TestMarkSynced(t *testing.T) {
	ctx := context.Background()
	mockSvc := &mockRAGSyncService{}

	err := mockSvc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockSvc.markedIDs) != 2 || mockSvc.markedIDs[0] != "1" {
		t.Errorf("unexpected marked IDs: %v", mockSvc.markedIDs)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	mockSvc := &mockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test1"},
	}

	err := mockSvc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockSvc.processedRecs) != 1 || mockSvc.processedRecs[0].ID != "1" {
		t.Errorf("unexpected processed records: %v", mockSvc.processedRecs)
	}
}

func TestProcessIncomingSyncError(t *testing.T) {
	ctx := context.Background()
	mockSvc := &mockRAGSyncService{
		processErr: errors.New("process error"),
	}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test1"},
	}

	err := mockSvc.ProcessIncomingSync(ctx, records)
	if err == nil || err.Error() != "process error" {
		t.Fatalf("expected 'process error', got: %v", err)
	}
}
