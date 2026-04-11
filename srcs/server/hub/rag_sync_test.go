package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
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
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.Incoming = append(m.Incoming, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		mock := &MockRAGSyncService{
			PendingRecords: []RAGSyncRecord{
				{ID: "1", SyncStatus: SyncStatusPending},
				{ID: "2", SyncStatus: SyncStatusPending},
			},
		}

		records, err := mock.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(records) != 2 {
			t.Fatalf("expected 2 records, got %d", len(records))
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		mock := &MockRAGSyncService{}
		err := mock.MarkSynced(ctx, []string{"1", "2"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(mock.MarkedSynced) != 2 {
			t.Fatalf("expected 2 records marked, got %d", len(mock.MarkedSynced))
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		mock := &MockRAGSyncService{}
		records := []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
		}
		err := mock.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(mock.Incoming) != 1 {
			t.Fatalf("expected 1 record processed, got %d", len(mock.Incoming))
		}
	})

	t.Run("RecordMetrics", func(t *testing.T) {
		RecordRAGSyncSuccess(ctx, 5)
		RecordRAGSyncError(ctx, 2)
	})

	t.Run("FetchError", func(t *testing.T) {
		expectedErr := errors.New("db connection failed")
		mock := &MockRAGSyncService{
			FetchErr: expectedErr,
		}

		_, err := mock.FetchPendingSyncs(ctx, 10)
		if err != expectedErr {
			t.Fatalf("expected error %v, got %v", expectedErr, err)
		}
	})
}
