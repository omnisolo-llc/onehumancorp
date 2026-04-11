package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRAGSyncService struct {
	records []hub.RAGSyncRecord
	synced  []string
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &mockRAGSyncService{
		records: []hub.RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test",
				Vector:     []byte{1, 2, 3},
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if len(svc.synced) != 1 || svc.synced[0] != "1" {
		t.Fatalf("expected record 1 to be synced")
	}

	err = svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
}
