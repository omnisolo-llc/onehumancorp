package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockRAGSyncService is a mock implementation of RAGSyncService.
type MockRAGSyncService struct {
	mock.Mock
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	args := m.Called(ctx, limit)
	return args.Get(0).([]RAGSyncRecord), args.Error(1)
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	args := m.Called(ctx, ids)
	return args.Error(0)
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	args := m.Called(ctx, records)
	return args.Error(0)
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := new(MockRAGSyncService)
	ctx := context.Background()

	expectedRecords := []RAGSyncRecord{
		{
			ID:         "rec1",
			Context:    "test context",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Time{},
		},
	}

	mockService.On("FetchPendingSyncs", ctx, 10).Return(expectedRecords, nil)

	records, err := mockService.FetchPendingSyncs(ctx, 10)

	assert.NoError(t, err)
	assert.Len(t, records, 1)
	assert.Equal(t, "rec1", records[0].ID)
	assert.Equal(t, SyncStatusPending, records[0].SyncStatus)

	mockService.AssertExpectations(t)
}

func TestMarkSynced(t *testing.T) {
	mockService := new(MockRAGSyncService)
	ctx := context.Background()

	ids := []string{"rec1", "rec2"}

	mockService.On("MarkSynced", ctx, ids).Return(nil)

	err := mockService.MarkSynced(ctx, ids)

	assert.NoError(t, err)

	mockService.AssertExpectations(t)
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := new(MockRAGSyncService)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "rec1",
			Context:    "test context",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
		},
	}

	mockService.On("ProcessIncomingSync", ctx, records).Return(nil)

	err := mockService.ProcessIncomingSync(ctx, records)

	assert.NoError(t, err)

	mockService.AssertExpectations(t)
}

func TestProcessIncomingSync_Error(t *testing.T) {
	mockService := new(MockRAGSyncService)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "rec1",
			Context:    "test context",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
		},
	}

	expectedError := errors.New("db error")
	mockService.On("ProcessIncomingSync", ctx, records).Return(expectedError)

	err := mockService.ProcessIncomingSync(ctx, records)

	assert.ErrorIs(t, err, expectedError)

	mockService.AssertExpectations(t)
}

func TestMetricsFunctions(t *testing.T) {
	// Simple test to ensure the metric functions don't panic
	ctx := context.Background()
	RecordSyncSuccess(ctx, 5)
	RecordSyncError(ctx, 1)
}
