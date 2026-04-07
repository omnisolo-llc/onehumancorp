package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/testutil"
)

// mockRAGSyncService is a mock implementation of RAGSyncService for testing
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
			RAGRecordsSyncedTotal.Inc()
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) > 0 && records[0].ID == "error-id" {
		RAGSyncErrorsTotal.Inc()
		return errors.New("mock error")
	}

	for _, incoming := range records {
		found := false
		for i, existing := range m.records {
			if existing.ID == incoming.ID {
				m.records[i] = incoming
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
				found = true
				break
			}
		}
		if !found {
			incoming.SyncStatus = SyncStatusSynced
			incoming.LastSyncAt = time.Now()
			m.records = append(m.records, incoming)
		}
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	// Reset metrics
	registry := prometheus.NewRegistry()
	prometheus.DefaultRegisterer = registry
	prometheus.DefaultGatherer = registry

	// Re-register the metrics we use in this test
	prometheus.MustRegister(RAGRecordsSyncedTotal)
	prometheus.MustRegister(RAGSyncErrorsTotal)

	ctx := context.Background()
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "context 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "context 3", SyncStatus: SyncStatusSynced},
		},
	}

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if testutil.ToFloat64(RAGRecordsSyncedTotal) != 1 {
		t.Errorf("expected 1 record synced metric, got %v", testutil.ToFloat64(RAGRecordsSyncedTotal))
	}

	pending, _ = mockService.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record after marking one synced, got %d", len(pending))
	}

	// Test ProcessIncomingSync - success
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "context 4"},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 4 {
		t.Errorf("expected 4 records, got %d", len(mockService.records))
	}

	// Test ProcessIncomingSync - error
	errorIncoming := []RAGSyncRecord{
		{ID: "error-id", Context: "error context"},
	}
	err = mockService.ProcessIncomingSync(ctx, errorIncoming)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if testutil.ToFloat64(RAGSyncErrorsTotal) != 1 {
		t.Errorf("expected 1 sync error metric, got %v", testutil.ToFloat64(RAGSyncErrorsTotal))
	}
}
