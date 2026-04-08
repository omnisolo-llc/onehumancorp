package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []RAGSyncRecord
	FetchError     error
	MarkError      error
	ProcessError   error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchError != nil {
		return nil, m.FetchError
	}
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkError != nil {
		return m.MarkError
	}
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessError != nil {
		return m.ProcessError
	}
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	svc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "1",
				Context:    "Summary of meeting",
				Vector:     []float32{0.1, 0.2, 0.3},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
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

func TestRAGSyncService_MarkSynced(t *testing.T) {
	ctx := context.Background()
	svc := &MockRAGSyncService{}

	err := svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(svc.MarkedIDs) != 2 {
		t.Fatalf("expected 2 marked IDs, got %d", len(svc.MarkedIDs))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	svc := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "Important cloud task",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusPending,
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(svc.ProcessedData) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(svc.ProcessedData))
	}
}

func TestRAGSyncService_Errors(t *testing.T) {
	ctx := context.Background()
	expectedErr := errors.New("simulated error")
	svc := &MockRAGSyncService{
		FetchError:   expectedErr,
		MarkError:    expectedErr,
		ProcessError: expectedErr,
	}

	_, err := svc.FetchPendingSyncs(ctx, 10)
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err != expectedErr {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}
}
