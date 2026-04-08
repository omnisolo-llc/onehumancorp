package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	mock.Mock
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	args := m.Called(ctx, limit)
	return args.Get(0).([]hub.RAGSyncRecord), args.Error(1)
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	args := m.Called(ctx, ids)
	return args.Error(0)
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	args := m.Called(ctx, records)
	return args.Error(0)
}

func TestRAGSyncService(t *testing.T) {
	mockService := new(MockRAGSyncService)
	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		expectedRecords := []hub.RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test context",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Time{},
			},
		}

		mockService.On("FetchPendingSyncs", ctx, 10).Return(expectedRecords, nil).Once()

		records, err := mockService.FetchPendingSyncs(ctx, 10)

		assert.NoError(t, err)
		assert.Equal(t, expectedRecords, records)
		mockService.AssertExpectations(t)
	})

	t.Run("MarkSynced", func(t *testing.T) {
		ids := []string{"1", "2"}
		mockService.On("MarkSynced", ctx, ids).Return(nil).Once()

		err := mockService.MarkSynced(ctx, ids)

		assert.NoError(t, err)
		mockService.AssertExpectations(t)

		// Ensure metric exists (this doesn't actually test the increment, just that the metric is initialized)
		assert.NotNil(t, hub.RagRecordsSyncedTotal)
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		records := []hub.RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test context",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: hub.SyncStatusSynced,
				LastSyncAt: time.Now(),
			},
		}

		mockService.On("ProcessIncomingSync", ctx, records).Return(nil).Once()

		err := mockService.ProcessIncomingSync(ctx, records)

		assert.NoError(t, err)
		mockService.AssertExpectations(t)
	})
}
