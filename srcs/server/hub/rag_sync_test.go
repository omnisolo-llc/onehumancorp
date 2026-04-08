package hub

import (
	"context"
	"errors"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	ProcessedRecords []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
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
	m.ProcessedRecords = append(m.ProcessedRecords, records...)
	return nil
}

func TestMockRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	mock.FetchErr = errors.New("fetch error")
	_, err = mock.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestMockRAGSyncService_MarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{}
	ids := []string{"1", "2"}

	err := mock.MarkSynced(context.Background(), ids)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.SyncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mock.SyncedIDs))
	}

	mock.MarkErr = errors.New("mark error")
	err = mock.MarkSynced(context.Background(), ids)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestMockRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusPending},
	}

	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.ProcessedRecords) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(mock.ProcessedRecords))
	}

	mock.ProcessErr = errors.New("process error")
	err = mock.ProcessIncomingSync(context.Background(), records)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}
