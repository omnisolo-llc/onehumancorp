package hub

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

type MockRAGSyncService struct {
	FetchPendingSyncsFunc   func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSyncedFunc          func(ctx context.Context, ids []string) error
	ProcessIncomingSyncFunc func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchPendingSyncsFunc != nil {
		return m.FetchPendingSyncsFunc(ctx, limit)
	}
	return nil, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkSyncedFunc != nil {
		return m.MarkSyncedFunc(ctx, ids)
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessIncomingSyncFunc != nil {
		return m.ProcessIncomingSyncFunc(ctx, records)
	}
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		mock := &MockRAGSyncService{
			FetchPendingSyncsFunc: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
				return []RAGSyncRecord{
					{ID: "1", SyncStatus: SyncStatusPending},
				}, nil
			},
		}

		records, err := mock.FetchPendingSyncs(ctx, 10)
		assert.NoError(t, err)
		assert.Len(t, records, 1)
		assert.Equal(t, "1", records[0].ID)
		assert.Equal(t, SyncStatusPending, records[0].SyncStatus)
	})

	t.Run("MarkSynced", func(t *testing.T) {
		called := false
		mock := &MockRAGSyncService{
			MarkSyncedFunc: func(ctx context.Context, ids []string) error {
				called = true
				assert.Equal(t, []string{"1", "2"}, ids)
				return nil
			},
		}

		err := mock.MarkSynced(ctx, []string{"1", "2"})
		assert.NoError(t, err)
		assert.True(t, called)
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		called := false
		mock := &MockRAGSyncService{
			ProcessIncomingSyncFunc: func(ctx context.Context, records []RAGSyncRecord) error {
				called = true
				assert.Len(t, records, 1)
				assert.Equal(t, "test-context", records[0].Context)
				return nil
			},
		}

		err := mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test-context",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		})
		assert.NoError(t, err)
		assert.True(t, called)
	})
}
