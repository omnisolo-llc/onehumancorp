package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	markSyncedErr  error
	processErr     error
	syncedIDs      []string
	processed      []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.pendingRecords) {
		return m.pendingRecords[:limit], nil
	}
	return m.pendingRecords, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.markSyncedErr != nil {
		return m.markSyncedErr
	}
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.processErr != nil {
		return m.processErr
	}
	m.processed = append(m.processed, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	now := time.Now()
	mockSvc := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending, LastSyncAt: now},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending, LastSyncAt: now},
		},
	}

	ctx := context.Background()
	records, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got: %d", len(records))
	}
	if records[0].ID != "1" || records[1].ID != "2" {
		t.Errorf("unexpected record IDs")
	}
}

func TestMarkSynced(t *testing.T) {
	mockSvc := &mockRAGSyncService{}
	ctx := context.Background()

	err := mockSvc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if len(mockSvc.syncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got: %d", len(mockSvc.syncedIDs))
	}
	if mockSvc.syncedIDs[0] != "1" || mockSvc.syncedIDs[1] != "2" {
		t.Errorf("unexpected synced IDs")
	}

	mockSvc.markSyncedErr = errors.New("db error")
	err = mockSvc.MarkSynced(ctx, []string{"3"})
	if err == nil || err.Error() != "db error" {
		t.Fatalf("expected db error, got: %v", err)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockSvc := &mockRAGSyncService{}
	ctx := context.Background()

	now := time.Now()
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusPending, LastSyncAt: now},
	}

	err := mockSvc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if len(mockSvc.processed) != 1 {
		t.Fatalf("expected 1 processed record, got: %d", len(mockSvc.processed))
	}
	if mockSvc.processed[0].ID != "1" {
		t.Errorf("unexpected processed record ID")
	}

	mockSvc.processErr = errors.New("db error")
	err = mockSvc.ProcessIncomingSync(ctx, records)
	if err == nil || err.Error() != "db error" {
		t.Fatalf("expected db error, got: %v", err)
	}
}
