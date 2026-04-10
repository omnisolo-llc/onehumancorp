package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	Records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return m.Records, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.Records {
		for _, id := range ids {
			if m.Records[i].ID == id {
				m.Records[i].SyncStatus = SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{}

	record := RAGSyncRecord{
		ID:         "test-id",
		Context:    "test-context",
		Vector:     []float32{0.1, 0.2},
		SyncStatus: SyncStatusPending,
		LastSyncAt: time.Now(),
	}

	err := mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	err = mock.MarkSynced(context.Background(), []string{"test-id"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.Records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected status synced")
	}
}
