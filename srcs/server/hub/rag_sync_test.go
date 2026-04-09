package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
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
		return pending[:limit], nil
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
		r.SyncStatus = SyncStatusSynced
		r.LastSyncAt = time.Now()
		m.records = append(m.records, r)
	}
	return nil
}

func TestRAGSyncService_Flow(t *testing.T) {
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test Context 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "Test Context 3", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, pending, 2)
	assert.Equal(t, "1", pending[0].ID)

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	assert.NoError(t, err)

	pendingAfterMark, err := mockService.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, pendingAfterMark, 1)
	assert.Equal(t, "2", pendingAfterMark[0].ID)

	// Verify sync status updated
	assert.Equal(t, SyncStatusSynced, mockService.records[0].SyncStatus)

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "Incoming Context 4"},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	assert.NoError(t, err)

	assert.Len(t, mockService.records, 4)
	assert.Equal(t, "4", mockService.records[3].ID)
	assert.Equal(t, SyncStatusSynced, mockService.records[3].SyncStatus)
}
