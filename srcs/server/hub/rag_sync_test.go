package hub

import (
	"context"
	"testing"
)

func TestInitRAGSyncMetrics_NilMeter(t *testing.T) {
	// Should not panic when meter is nil
	InitRAGSyncMetrics(nil)
}

type MockRAGSyncService struct {
	FetchPendingSyncsFn   func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSyncedFn          func(ctx context.Context, ids []string) error
	ProcessIncomingSyncFn func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchPendingSyncsFn != nil {
		return m.FetchPendingSyncsFn(ctx, limit)
	}
	return nil, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkSyncedFn != nil {
		return m.MarkSyncedFn(ctx, ids)
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessIncomingSyncFn != nil {
		return m.ProcessIncomingSyncFn(ctx, records)
	}
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mock := &MockRAGSyncService{
		FetchPendingSyncsFn: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
			return []RAGSyncRecord{{ID: "1"}}, nil
		},
		MarkSyncedFn: func(ctx context.Context, ids []string) error {
			if len(ids) != 1 || ids[0] != "1" {
				t.Errorf("Expected id 1, got %v", ids)
			}
			return nil
		},
		ProcessIncomingSyncFn: func(ctx context.Context, records []RAGSyncRecord) error {
			if len(records) != 1 || records[0].ID != "1" {
				t.Errorf("Expected record id 1, got %v", records)
			}
			return nil
		},
	}

	ctx := context.Background()
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil || len(records) != 1 || records[0].ID != "1" {
		t.Errorf("FetchPendingSyncs failed")
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Errorf("MarkSynced failed")
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "1"}})
	if err != nil {
		t.Errorf("ProcessIncomingSync failed")
	}
}
