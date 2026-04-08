package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	processed      []RAGSyncRecord
	err            error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.err != nil {
		return nil, m.err
	}
	if limit > len(m.pendingRecords) {
		limit = len(m.pendingRecords)
	}
	return m.pendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.err != nil {
		return m.err
	}
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.err != nil {
		return m.err
	}
	m.processed = append(m.processed, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{}

	err := mock.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mock.syncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mock.syncedIDs))
	}
	if mock.syncedIDs[0] != "1" || mock.syncedIDs[1] != "2" {
		t.Errorf("expected IDs [1, 2], got %v", mock.syncedIDs)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}
	now := time.Now()
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusPending, LastSyncAt: now},
	}

	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mock.processed) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(mock.processed))
	}
	if mock.processed[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", mock.processed[0].ID)
	}
}

func TestMockError(t *testing.T) {
	expectedErr := errors.New("mock error")
	mock := &MockRAGSyncService{err: expectedErr}

	_, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = mock.MarkSynced(context.Background(), []string{"1"})
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}
}
