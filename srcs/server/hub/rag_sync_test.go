package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	Incoming       []RAGSyncRecord
	ErrToReturn    error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.ErrToReturn != nil {
		RecordSyncError(ctx)
		return nil, m.ErrToReturn
	}

	count := limit
	if count > len(m.PendingRecords) {
		count = len(m.PendingRecords)
	}
	res := m.PendingRecords[:count]
	m.PendingRecords = m.PendingRecords[count:]

	return res, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.ErrToReturn != nil {
		RecordSyncError(ctx)
		return m.ErrToReturn
	}
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	RecordSyncSuccess(ctx, len(ids))
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ErrToReturn != nil {
		RecordSyncError(ctx)
		return m.ErrToReturn
	}
	m.Incoming = append(m.Incoming, records...)
	RecordSyncSuccess(ctx, len(records))
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 2)
	assert.Equal(t, "1", records[0].ID)
	assert.Equal(t, "2", records[1].ID)
}

func TestMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ctx := context.Background()

	err := mockService.MarkSynced(ctx, []string{"1", "2"})
	assert.NoError(t, err)
	assert.Len(t, mockService.SyncedIDs, 2)
	assert.Contains(t, mockService.SyncedIDs, "1")
	assert.Contains(t, mockService.SyncedIDs, "2")
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, records)
	assert.NoError(t, err)
	assert.Len(t, mockService.Incoming, 1)
	assert.Equal(t, "1", mockService.Incoming[0].ID)
}

func TestRecordSyncMetrics(t *testing.T) {
	// This test simply ensures the metrics functions can be called without panicking.
	ctx := context.Background()
	RecordSyncSuccess(ctx, 5)
	RecordSyncError(ctx)
	RecordSyncLatency(ctx, time.Second)
	RecordPendingCount(ctx, 10)
}
