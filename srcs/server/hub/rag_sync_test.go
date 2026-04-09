package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of the RAGSyncService interface.
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	Incoming       []RAGSyncRecord
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
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.Incoming = append(m.Incoming, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	records := []RAGSyncRecord{
		{ID: "1", Context: "Test 1", SyncStatus: SyncStatusPending},
		{ID: "2", Context: "Test 2", SyncStatus: SyncStatusPending},
	}
	mockSvc := &MockRAGSyncService{PendingRecords: records}

	fetched, err := mockSvc.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(fetched) != 1 {
		t.Fatalf("expected 1 record, got %d", len(fetched))
	}
	if fetched[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", fetched[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	ctx := context.Background()
	mockSvc := &MockRAGSyncService{}
	ids := []string{"1", "2"}

	err := mockSvc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.MarkedIDs) != 2 {
		t.Fatalf("expected 2 marked IDs, got %d", len(mockSvc.MarkedIDs))
	}
	if mockSvc.MarkedIDs[0] != "1" || mockSvc.MarkedIDs[1] != "2" {
		t.Errorf("unexpected marked IDs: %v", mockSvc.MarkedIDs)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	mockSvc := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "3", Context: "Test 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := mockSvc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.Incoming) != 1 {
		t.Fatalf("expected 1 incoming record, got %d", len(mockSvc.Incoming))
	}
	if mockSvc.Incoming[0].ID != "3" {
		t.Errorf("expected ID 3, got %s", mockSvc.Incoming[0].ID)
	}
}

func TestFetchPendingSyncs_Error(t *testing.T) {
	ctx := context.Background()
	expectedErr := errors.New("db connection failed")
	mockSvc := &MockRAGSyncService{FetchErr: expectedErr}

	_, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	svc := NewDefaultRAGSyncService()

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if records != nil {
		t.Fatalf("expected nil records, got %v", records)
	}
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	ctx := context.Background()
	svc := NewDefaultRAGSyncService()

	err := svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	svc := NewDefaultRAGSyncService()

	err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
