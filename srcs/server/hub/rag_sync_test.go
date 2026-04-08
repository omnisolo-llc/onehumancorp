package hub

import (
	"context"
	"errors"
	"testing"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	processed      []RAGSyncRecord
	fetchErr       error
	markErr        error
	processErr     error
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.fetchErr != nil {
		return nil, m.fetchErr
	}
	if len(m.pendingRecords) > limit {
		return m.pendingRecords[:limit], nil
	}
	return m.pendingRecords, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.markErr != nil {
		return m.markErr
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

func TestSyncLoop_Success(t *testing.T) {
	mock := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	err := SyncLoop(context.Background(), mock)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.processed) != 2 {
		t.Errorf("expected 2 records to be processed, got %d", len(mock.processed))
	}
	if len(mock.syncedIDs) != 2 {
		t.Errorf("expected 2 records to be marked synced, got %d", len(mock.syncedIDs))
	}
	if mock.syncedIDs[0] != "1" || mock.syncedIDs[1] != "2" {
		t.Errorf("unexpected synced IDs: %v", mock.syncedIDs)
	}
}

func TestSyncLoop_FetchError(t *testing.T) {
	mock := &mockRAGSyncService{
		fetchErr: errors.New("fetch error"),
	}

	err := SyncLoop(context.Background(), mock)
	if err == nil || err.Error() != "fetch error" {
		t.Fatalf("expected fetch error, got %v", err)
	}
}

func TestSyncLoop_ProcessError(t *testing.T) {
	mock := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
		},
		processErr: errors.New("process error"),
	}

	err := SyncLoop(context.Background(), mock)
	if err == nil || err.Error() != "process error" {
		t.Fatalf("expected process error, got %v", err)
	}
	if len(mock.syncedIDs) != 0 {
		t.Errorf("expected 0 records to be marked synced, got %d", len(mock.syncedIDs))
	}
}

func TestSyncLoop_MarkError(t *testing.T) {
	mock := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
		},
		markErr: errors.New("mark error"),
	}

	err := SyncLoop(context.Background(), mock)
	if err == nil || err.Error() != "mark error" {
		t.Fatalf("expected mark error, got %v", err)
	}
	if len(mock.processed) != 1 {
		t.Errorf("expected 1 record to be processed, got %d", len(mock.processed))
	}
}
