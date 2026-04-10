package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

// MockProvider embeds db.Provider to fulfill the interface
type MockProvider struct {
	db.Provider
	execCount int
}

func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
	return &MockTx{m}, nil
}

func (m *MockProvider) Query(ctx context.Context, sqlQuery string, args ...any) (db.Rows, error) {
	return &MockRows{}, nil
}

type MockTx struct {
	p *MockProvider
}

func (m *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.p.execCount++
	return 1, nil
}

func (m *MockTx) Commit(ctx context.Context) error {
	return nil
}

func (m *MockTx) Rollback(ctx context.Context) error {
	return nil
}

func (m *MockTx) Query(ctx context.Context, sql string, args ...any) (db.Rows, error) {
	return &MockRows{}, nil
}

func (m *MockTx) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	return nil
}

type MockRows struct{}

func (m *MockRows) Next() bool {
	return false
}

func (m *MockRows) Scan(dest ...any) error {
	return nil
}

func (m *MockRows) Close() {}

func (m *MockRows) Columns() ([]string, error) {
    return []string{}, nil
}

func (m *MockRows) Err() error {
	return nil
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	mockDB := &MockProvider{}
	svc := hub.NewDefaultRAGSyncService(mockDB)

	ctx := context.Background()
	ids := []string{"1", "2"}
	err := svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockDB.execCount != 2 {
		t.Errorf("expected 2 execs, got %d", mockDB.execCount)
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockDB := &MockProvider{}
	svc := hub.NewDefaultRAGSyncService(mockDB)

	ctx := context.Background()
	records := []hub.RAGSyncRecord{
		{ID: "1", Context: "ctx", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockDB.execCount != 1 {
		t.Errorf("expected 1 exec, got %d", mockDB.execCount)
	}
}
