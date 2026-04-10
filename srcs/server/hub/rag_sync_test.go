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
	for i := range m.records {
		for _, id := range ids {
			if m.records[i].ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &mockRAGSyncService{}

	record := RAGSyncRecord{
		ID:         "test-id",
		Context:    "test context",
		SyncStatus: SyncStatusPending,
		LastSyncAt: time.Now(),
	}

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pending, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.MarkSynced(context.Background(), []string{"test-id"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if svc.records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected status synced, got %v", svc.records[0].SyncStatus)
	}
}
