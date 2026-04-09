package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockProvider struct {
	ExecCount int
	QueryCount int
	BeginCount int
	isSQLite bool
}

func (m *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.ExecCount++
	return 1, nil
}
func (m *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	m.QueryCount++
	return &MockRows{}, nil
}
func (m *MockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}
func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
	m.BeginCount++
	return &MockTx{p: m}, nil
}
func (m *MockProvider) Close() {}
func (m *MockProvider) IsSQLite() bool {
	return m.isSQLite
}
func (m *MockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

type MockRows struct {
	count int
}
func (m *MockRows) Next() bool {
	m.count++
	return m.count <= 1 // return 1 row
}
func (m *MockRows) Scan(dest ...any) error {
	// mock scan data
	return nil
}
func (m *MockRows) Close() {}
func (m *MockRows) Columns() ([]string, error) { return nil, nil }
func (m *MockRows) Err() error { return nil }

type MockTx struct {
	p *MockProvider
}
func (m *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.p.ExecCount++
	return 1, nil
}
func (m *MockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, nil
}
func (m *MockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}
func (m *MockTx) Commit(ctx context.Context) error { return nil }
func (m *MockTx) Rollback(ctx context.Context) error { return nil }

func TestProviderRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	mockProvider := &MockProvider{isSQLite: true}
	service := NewProviderRAGSyncService(mockProvider)

	records := []RAGSyncRecord{
		{
			ID: "test_1",
			Context: "some context",
			Vector: []float32{1.0, 2.0},
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockProvider.BeginCount != 1 {
		t.Errorf("expected Begin to be called 1 time, got %d", mockProvider.BeginCount)
	}
	if mockProvider.ExecCount != 1 {
		t.Errorf("expected Exec to be called 1 time, got %d", mockProvider.ExecCount)
	}
}

func TestProviderRAGSyncService_MarkSynced(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	mockProvider := &MockProvider{isSQLite: true}
	service := NewProviderRAGSyncService(mockProvider)

	err := service.MarkSynced(ctx, []string{"test_1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockProvider.BeginCount != 1 {
		t.Errorf("expected Begin to be called 1 time, got %d", mockProvider.BeginCount)
	}
	if mockProvider.ExecCount != 1 {
		t.Errorf("expected Exec to be called 1 time, got %d", mockProvider.ExecCount)
	}
}

func TestProviderRAGSyncService_FetchPendingSyncs(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	mockProvider := &MockProvider{isSQLite: true}
	service := NewProviderRAGSyncService(mockProvider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockProvider.QueryCount != 1 {
		t.Errorf("expected Query to be called 1 time, got %d", mockProvider.QueryCount)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record returned, got %d", len(records))
	}
}
