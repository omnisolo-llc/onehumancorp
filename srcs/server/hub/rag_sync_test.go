package hub_test

import (
    "context"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRow struct{}
func (m *mockRow) Scan(dest ...any) error { return nil }
type mockRows struct{}
func (m *mockRows) Next() bool { return false }
func (m *mockRows) Scan(dest ...any) error { return nil }
func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

type mockTx struct{}
func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 1, nil }
func (m *mockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return &mockRows{}, nil }
func (m *mockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return &mockRow{} }
func (m *mockTx) Commit(ctx context.Context) error { return nil }
func (m *mockTx) Rollback(ctx context.Context) error { return nil }

type mockProvider struct {
    db.Provider
}
func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &mockRows{}, nil
}
func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    return 1, nil
}
func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTx{}, nil
}
func (m *mockProvider) IsSQLite() bool {
    return false
}

func TestRAGSyncService(t *testing.T) {
    ctx := context.Background()
    provider := &mockProvider{}
    service := hub.NewRAGSyncService(provider)

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 records, got %d", len(records))
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Test ProcessIncomingSync
    err = service.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{{ID: "1", Context: "test"}})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
