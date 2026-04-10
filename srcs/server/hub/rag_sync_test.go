package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"go.opentelemetry.io/otel/metric/noop"
)

// MockProvider mocks db.Provider
type MockProvider struct {
	mock.Mock
}

func (m *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	args := m.Called(ctx, sql, arguments)
	return args.Get(0).(int64), args.Error(1)
}

func (m *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	args := m.Called(ctx, sql, optionsAndArgs)
	if rows, ok := args.Get(0).(db.Rows); ok {
		return rows, args.Error(1)
	}
	return nil, args.Error(1)
}

func (m *MockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	args := m.Called(ctx, sql, optionsAndArgs)
	return args.Get(0).(db.Row)
}

func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
	args := m.Called(ctx)
	if tx, ok := args.Get(0).(db.Tx); ok {
		return tx, args.Error(1)
	}
	return nil, args.Error(1)
}

func (m *MockProvider) Close() {
	m.Called()
}

// MockTx mocks db.Tx
type MockTx struct {
	mock.Mock
}

func (m *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	args := m.Called(ctx, sql, arguments)
	return args.Get(0).(int64), args.Error(1)
}

func (m *MockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	args := m.Called(ctx, sql, optionsAndArgs)
	if rows, ok := args.Get(0).(db.Rows); ok {
		return rows, args.Error(1)
	}
	return nil, args.Error(1)
}

func (m *MockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	args := m.Called(ctx, sql, optionsAndArgs)
	return args.Get(0).(db.Row)
}

func (m *MockTx) Commit(ctx context.Context) error {
	return m.Called(ctx).Error(0)
}

func (m *MockTx) Rollback(ctx context.Context) error {
	return m.Called(ctx).Error(0)
}

// MockRows mocks db.Rows
type MockRows struct {
	mock.Mock
}

func (m *MockRows) Next() bool {
	return m.Called().Bool(0)
}

func (m *MockRows) Scan(dest ...any) error {
	args := m.Called(dest)
	return args.Error(0)
}

func (m *MockRows) Close() {
	m.Called()
}

func (m *MockRows) Columns() ([]string, error) {
	args := m.Called()
	return args.Get(0).([]string), args.Error(1)
}

func (m *MockRows) Err() error {
	return m.Called().Error(0)
}


func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockDB := new(MockProvider)
	mockTx := new(MockTx)

	meter := noop.NewMeterProvider().Meter("test")
	service, err := NewRAGSyncService(mockDB, meter)
	assert.NoError(t, err)

	ctx := context.Background()
	now := time.Now()

	records := []RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "test",
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
	}

	mockDB.On("Begin", ctx).Return(mockTx, nil)
	mockTx.On("Exec", ctx, mock.AnythingOfType("string"), []any{"mem1", "test", SyncStatusPending, now}).Return(int64(1), nil)
	mockTx.On("Commit", ctx).Return(nil)
	mockTx.On("Rollback", ctx).Return(nil)

	err = service.ProcessIncomingSync(ctx, records)
	assert.NoError(t, err)
	mockDB.AssertExpectations(t)
	mockTx.AssertExpectations(t)
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mockDB := new(MockProvider)
	mockTx := new(MockTx)

	meter := noop.NewMeterProvider().Meter("test")
	service, err := NewRAGSyncService(mockDB, meter)
	assert.NoError(t, err)

	ctx := context.Background()

	mockDB.On("Begin", ctx).Return(mockTx, nil)
	mockTx.On("Exec", ctx, mock.AnythingOfType("string"), mock.Anything).Return(int64(1), nil)
	mockTx.On("Commit", ctx).Return(nil)
	mockTx.On("Rollback", ctx).Return(nil)

	err = service.MarkSynced(ctx, []string{"mem1"})
	assert.NoError(t, err)
	mockDB.AssertExpectations(t)
	mockTx.AssertExpectations(t)
}

func (m *MockProvider) IsSQLite() bool {
	return m.Called().Bool(0)
}

func (m *MockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	args := m.Called(ctx, agentID)
	if tr, ok := args.Get(0).(*db.TaskRecord); ok {
		return tr, args.Error(1)
	}
	return nil, args.Error(1)
}
