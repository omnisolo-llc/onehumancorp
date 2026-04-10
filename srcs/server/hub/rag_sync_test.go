package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	incomingSyncs  []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pendingRecords) {
		limit = len(m.pendingRecords)
	}
	res := m.pendingRecords[:limit]
	m.pendingRecords = m.pendingRecords[limit:]
	return res, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingSyncs = append(m.incomingSyncs, records...)
	return nil
}

func TestRAGSyncInterface(t *testing.T) {
	ctx := context.Background()
	svc := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "context 2", SyncStatus: SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 1)
	assert.NoError(t, err)
	assert.Len(t, records, 1)
	assert.Equal(t, "1", records[0].ID)

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	assert.NoError(t, err)
	assert.Contains(t, svc.syncedIDs, "1")

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "3", Context: "incoming context", LastSyncAt: time.Now()},
	}
	err = svc.ProcessIncomingSync(ctx, newRecords)
	assert.NoError(t, err)
	assert.Len(t, svc.incomingSyncs, 1)
	assert.Equal(t, "3", svc.incomingSyncs[0].ID)
}
