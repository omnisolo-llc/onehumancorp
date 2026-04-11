package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		if rec.Context == "" {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return errors.New("empty context not allowed")
		}
	}
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestRAGSyncService_FetchAndMark(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "rec1", Context: "test context", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Ensure metrics are initialized
	// The InitWithMeter uses the actual global var, we can just ensure they aren't nil inside our tests if we call telemetry init or just let the nil check handle it.

	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "rec1" {
		t.Errorf("expected ID rec1, got %s", records[0].ID)
	}

	idsToMark := []string{records[0].ID}
	err = mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if len(mockService.MarkedSynced) != 1 || mockService.MarkedSynced[0] != "rec1" {
		t.Errorf("expected ID rec1 to be marked synced")
	}
}

func TestRAGSyncService_ProcessIncoming(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ctx := context.Background()

	validRecords := []RAGSyncRecord{
		{ID: "rec2", Context: "test incoming", LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, validRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(mockService.ProcessedSyncs) != 1 || mockService.ProcessedSyncs[0].ID != "rec2" {
		t.Errorf("expected rec2 to be processed")
	}

	invalidRecords := []RAGSyncRecord{
		{ID: "rec3", Context: ""},
	}

	err = mockService.ProcessIncomingSync(ctx, invalidRecords)
	if err == nil {
		t.Errorf("expected error for empty context")
	}
}
