package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
    db.Provider
    execCalls int
    queryCalls int
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    m.queryCalls++
    return &mockRows{}, nil
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.execCalls++
    return 1, nil
}

type mockRows struct {
    count int
}
func (m *mockRows) Next() bool {
    if m.count < 2 {
        m.count++
        return true
    }
    return false
}
func (m *mockRows) Scan(dest ...any) error {
    return nil
}
func (m *mockRows) Close() {}
func (m *mockRows) Err() error { return nil }
func (m *mockRows) Columns() ([]string, error) { return nil, nil }

func TestSQLRAGSyncService(t *testing.T) {
    provider := &mockProvider{}
    service := NewSQLRAGSyncService(provider)

    records := []RAGSyncRecord{
        {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
        {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
    }

    err := service.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
    if provider.execCalls != 2 {
        t.Fatalf("Expected 2 exec calls, got %d", provider.execCalls)
    }

    _, err = service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if provider.queryCalls != 1 {
        t.Fatalf("Expected 1 query call, got %d", provider.queryCalls)
    }

    err = service.MarkSynced(context.Background(), []string{"1", "2"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    if provider.execCalls != 4 {
        t.Fatalf("Expected 4 total exec calls (2 insert + 2 update), got %d", provider.execCalls)
    }
}
