package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
	synced  []string
	err     error
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.err != nil {
		return nil, m.err
	}
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.err != nil {
		return m.err
	}
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.err != nil {
		return m.err
	}
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncDataFlow(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	ids := []string{"1", "2"}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.synced) != 2 {
		t.Fatalf("expected 2 synced records, got %d", len(mockService.synced))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "3", Context: "test context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 3 {
		t.Fatalf("expected 3 total records after incoming sync, got %d", len(mockService.records))
	}
}

func TestRAGSyncDataFlowError(t *testing.T) {
	expectedErr := errors.New("simulated error")
	mockService := &mockRAGSyncService{
		err: expectedErr,
	}

	ctx := context.Background()

	_, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != expectedErr {
		t.Fatalf("expected %v, got %v", expectedErr, err)
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != expectedErr {
		t.Fatalf("expected %v, got %v", expectedErr, err)
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err != expectedErr {
		t.Fatalf("expected %v, got %v", expectedErr, err)
	}
}
