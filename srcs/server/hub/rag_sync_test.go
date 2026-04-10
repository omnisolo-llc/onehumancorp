package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{}

	record := RAGSyncRecord{
		ID:         "test-id",
		Context:    "test context",
		SyncStatus: SyncStatusPending,
		LastSyncAt: time.Now(),
	}

	err := mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
}
