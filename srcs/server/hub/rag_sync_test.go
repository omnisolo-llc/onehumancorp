package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	Records        []RAGSyncRecord
	MarkedSynced   []string
	IncomingSyncs  []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if len(m.Records) > limit {
		return m.Records[:limit], nil
	}
	return m.Records, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.IncomingSyncs = append(m.IncomingSyncs, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	t.Run("FetchPendingSyncs", func(t *testing.T) {
		mockService := &MockRAGSyncService{
			Records: []RAGSyncRecord{
				{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
				{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
			},
		}

		records, err := mockService.FetchPendingSyncs(context.Background(), 10)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(records) != 2 {
			t.Fatalf("expected 2 records, got %d", len(records))
		}

		mockService.FetchErr = errors.New("fetch error")
		_, err = mockService.FetchPendingSyncs(context.Background(), 10)
		if err == nil {
			t.Fatal("expected error, got nil")
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		mockService := &MockRAGSyncService{}

		err := mockService.MarkSynced(context.Background(), []string{"1", "2"})
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(mockService.MarkedSynced) != 2 {
			t.Fatalf("expected 2 marked records, got %d", len(mockService.MarkedSynced))
		}

		mockService.MarkErr = errors.New("mark error")
		err = mockService.MarkSynced(context.Background(), []string{"3"})
		if err == nil {
			t.Fatal("expected error, got nil")
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		mockService := &MockRAGSyncService{}
		records := []RAGSyncRecord{
			{ID: "1", Context: "incoming1", LastSyncAt: time.Now()},
		}

		err := mockService.ProcessIncomingSync(context.Background(), records)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(mockService.IncomingSyncs) != 1 {
			t.Fatalf("expected 1 incoming record, got %d", len(mockService.IncomingSyncs))
		}

		mockService.ProcessErr = errors.New("process error")
		err = mockService.ProcessIncomingSync(context.Background(), records)
		if err == nil {
			t.Fatal("expected error, got nil")
		}
	})
}
