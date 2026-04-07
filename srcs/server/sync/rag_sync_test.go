package sync

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric/noop"
)

type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
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
	m.records = append(m.records, records...)
	return nil
}

func TestMockRAGSyncService_FetchPendingSyncs(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Errorf("expected ids 1 and 3, got %s and %s", pending[0].ID, pending[1].ID)
	}
}

func TestMockRAGSyncService_MarkSynced(t *testing.T) {
	svc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if svc.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced, got %s", svc.records[0].SyncStatus)
	}

	if svc.records[1].SyncStatus != SyncStatusPending {
		t.Errorf("expected record 2 to be pending, got %s", svc.records[1].SyncStatus)
	}
}

func TestMockRAGSyncService_ProcessIncomingSync(t *testing.T) {
	svc := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusSynced},
	}

	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(svc.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(svc.records))
	}

	if svc.records[0].ID != "1" {
		t.Errorf("expected id 1, got %s", svc.records[0].ID)
	}
}

func TestNewTelemetry(t *testing.T) {
	meter := noop.NewMeterProvider().Meter("test")
	telemetry, err := NewTelemetry(meter)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if telemetry.RecordsSyncedTotal == nil {
		t.Error("expected RecordsSyncedTotal to be initialized")
	}

	if telemetry.SyncErrorsTotal == nil {
		t.Error("expected SyncErrorsTotal to be initialized")
	}
}
