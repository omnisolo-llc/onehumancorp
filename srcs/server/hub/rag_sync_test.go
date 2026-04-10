package hub

import (
	"context"
	"errors"
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
		}
	}
	if len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
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

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		if r.ID == "error_id" {
			return errors.New("simulated error")
		}
	}
	// Upsert logic would go here. For mock we just append.
	m.records = append(m.records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}
	ctx := context.Background()
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}
	ctx := context.Background()
	err := svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if svc.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected status synced, got %v", svc.records[0].SyncStatus)
	}
	if svc.records[0].LastSyncAt.IsZero() {
		t.Errorf("expected last sync time to be set")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	svc := &MockRAGSyncService{}
	ctx := context.Background()
	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusPending},
	}
	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.records) != 1 {
		t.Errorf("expected 1 record, got %d", len(svc.records))
	}
}

func TestProcessIncomingSync_Error(t *testing.T) {
	svc := &MockRAGSyncService{}
	ctx := context.Background()
	records := []RAGSyncRecord{
		{ID: "error_id", SyncStatus: SyncStatusPending},
	}
	err := svc.ProcessIncomingSync(ctx, records)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
