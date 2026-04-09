package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing.
type MockRAGSyncService struct {
	FetchPendingSyncsFunc func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSyncedFunc        func(ctx context.Context, ids []string) error
	ProcessIncomingSyncFunc func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchPendingSyncsFunc != nil {
		return m.FetchPendingSyncsFunc(ctx, limit)
	}
	return nil, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkSyncedFunc != nil {
		return m.MarkSyncedFunc(ctx, ids)
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessIncomingSyncFunc != nil {
		return m.ProcessIncomingSyncFunc(ctx, records)
	}
	return nil
}

func TestMockRAGSyncService_FetchPendingSyncs(t *testing.T) {
	expectedRecords := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "test context 1",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Time{},
		},
	}

	mockSvc := &MockRAGSyncService{
		FetchPendingSyncsFunc: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
			if limit != 10 {
				t.Errorf("expected limit 10, got %d", limit)
			}
			return expectedRecords, nil
		},
	}

	records, err := mockSvc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected record ID '1', got '%s'", records[0].ID)
	}
}

func TestMockRAGSyncService_MarkSynced(t *testing.T) {
	mockSvc := &MockRAGSyncService{
		MarkSyncedFunc: func(ctx context.Context, ids []string) error {
			if len(ids) != 2 {
				t.Errorf("expected 2 ids, got %d", len(ids))
			}
			if ids[0] != "1" || ids[1] != "2" {
				t.Errorf("unexpected ids: %v", ids)
			}
			return nil
		},
	}

	err := mockSvc.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestMockRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockErr := errors.New("mock error")
	mockSvc := &MockRAGSyncService{
		ProcessIncomingSyncFunc: func(ctx context.Context, records []RAGSyncRecord) error {
			if len(records) != 1 {
				t.Errorf("expected 1 record, got %d", len(records))
			}
			return mockErr
		},
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "test context",
			SyncStatus: SyncStatusSynced,
		},
	}

	err := mockSvc.ProcessIncomingSync(context.Background(), records)
	if err != mockErr {
		t.Fatalf("expected mock error, got: %v", err)
	}
}

func TestMetricsInitialization(t *testing.T) {
	// Simple test to ensure the global metrics vars are initialized without panic.
	if RecordsSyncedTotal == nil {
		t.Fatal("RecordsSyncedTotal metric not initialized")
	}
	if SyncErrorsTotal == nil {
		t.Fatal("SyncErrorsTotal metric not initialized")
	}

	// We can add to the metrics in tests, though verifying the state of the global meter provider requires more setup.
	RecordsSyncedTotal.Add(context.Background(), 1)
	SyncErrorsTotal.Add(context.Background(), 1)
}
