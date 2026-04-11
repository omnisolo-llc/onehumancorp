package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
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
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	service := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, pending, 2)
	assert.Equal(t, "1", pending[0].ID)
	assert.Equal(t, "3", pending[1].ID)

	// Mark synced
	err = service.MarkSynced(ctx, []string{"1"})
	assert.NoError(t, err)

	// Fetch pending again
	pending, err = service.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, pending, 1)
	assert.Equal(t, "3", pending[0].ID)

	// Process incoming
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test4", SyncStatus: SyncStatusSynced},
	})
	assert.NoError(t, err)

	assert.Len(t, service.records, 4)
}
