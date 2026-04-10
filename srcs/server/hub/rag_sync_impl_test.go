package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

type mockTx struct {
	db.Tx
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, nil
}

func (m *mockTx) Commit(ctx context.Context) error {
	return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
	return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	return &mockTx{}, nil
}

type mockRows struct {
	count int
}

func (m *mockRows) Next() bool {
	if m.count > 0 {
		m.count--
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	// For testing just assign zero values
	return nil
}

func (m *mockRows) Close() {}

func (m *mockRows) Err() error {
	return nil
}

func (m *mockRows) Columns() ([]string, error) {
	return []string{"id", "content", "embedding", "sync_status", "last_sync_at"}, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{count: 2}, nil
}

func TestDefaultRAGSyncService(t *testing.T) {
	provider := &mockProvider{isSQLite: true}
	service := NewDefaultRAGSyncService(provider)

	ctx := context.Background()

	// Test Fetch
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test ProcessIncoming
	recs := []RAGSyncRecord{
		{ID: "3", Context: "test", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = service.ProcessIncomingSync(ctx, recs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
