package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MockRAGSyncService struct {
	fetchPendingSyncsFn   func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	markSyncedFn          func(ctx context.Context, ids []string) error
	processIncomingSyncFn func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.fetchPendingSyncsFn != nil {
		return m.fetchPendingSyncsFn(ctx, limit)
	}
	return nil, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.markSyncedFn != nil {
		return m.markSyncedFn(ctx, ids)
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.processIncomingSyncFn != nil {
		return m.processIncomingSyncFn(ctx, records)
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	cleanup, err := telemetry.InitTelemetry()
	assert.NoError(t, err)
	defer cleanup()

	mockSvc := &MockRAGSyncService{}
	ctx := context.Background()

	// Test data
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", Vector: []float32{1.0, 2.0}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	// Setup expectations
	mockSvc.fetchPendingSyncsFn = func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
		assert.Equal(t, 10, limit)
		return records, nil
	}

	mockSvc.processIncomingSyncFn = func(ctx context.Context, recs []RAGSyncRecord) error {
		assert.Len(t, recs, 1)
		return nil
	}

	mockSvc.markSyncedFn = func(ctx context.Context, ids []string) error {
		assert.Len(t, ids, 1)
		assert.Equal(t, "1", ids[0])
		return nil
	}

	// Execute flow
	fetched, err := mockSvc.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, fetched, 1)

	err = mockSvc.ProcessIncomingSync(ctx, fetched)
	assert.NoError(t, err)

	ids := []string{fetched[0].ID}
	err = mockSvc.MarkSynced(ctx, ids)
	assert.NoError(t, err)

	// Trigger telemetry to ensure no panic
	telemetry.RecordRagRecordSynced(ctx)
	telemetry.RecordRagSyncError(ctx)
}
