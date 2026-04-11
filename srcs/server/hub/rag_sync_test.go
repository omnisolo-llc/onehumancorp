package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"go.opentelemetry.io/otel"
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
	if len(records) > 10 {
		return errors.New("batch too large")
	}
	for _, r := range records {
		r.SyncStatus = SyncStatusSynced
		m.records = append(m.records, r)
	}
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	// Initialize metrics
	meter := otel.Meter("test-meter")
	err := InitMetrics(meter)
	if err != nil {
		t.Fatalf("Failed to init metrics: %v", err)
	}

	service := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", Vector: []byte{4, 5, 6}, SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 5)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Process incoming (Cloud side)
	err = service.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Mark synced (Local side)
	ids := []string{"1", "2"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify local state updated
	pendingAgain, _ := service.FetchPendingSyncs(ctx, 5)
	if len(pendingAgain) != 0 {
		t.Fatalf("Expected 0 pending records after sync, got %d", len(pendingAgain))
	}
}
