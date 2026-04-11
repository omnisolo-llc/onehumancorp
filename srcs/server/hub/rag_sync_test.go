package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if len(pending) >= limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, in := range records {
		found := false
		for i, r := range m.records {
			if r.ID == in.ID {
				m.records[i] = in
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
				found = true
				break
			}
		}
		if !found {
			in.SyncStatus = SyncStatusSynced
			in.LastSyncAt = time.Now()
			m.records = append(m.records, in)
		}
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Memory 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Memory 2", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending syncs, got %d", len(pending))
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending sync, got %d", len(pending))
	}

	err = svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "3", Context: "Memory 3", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	if len(svc.records) != 3 {
		t.Fatalf("Expected 3 records total, got %d", len(svc.records))
	}
}
