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
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		r.SyncStatus = SyncStatusSynced
		r.LastSyncAt = time.Now()
		m.records = append(m.records, r)
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	err := InitRAGSyncMetrics()
	if err != nil {
		t.Fatalf("Failed to init metrics: %v", err)
	}

	svc := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "2" {
		t.Errorf("Expected 1 pending record with ID '2', got %v", pending)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	if len(svc.records) != 4 {
		t.Errorf("Expected 4 total records, got %d", len(svc.records))
	}
}
