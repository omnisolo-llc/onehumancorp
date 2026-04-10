package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

// Mock DB Provider embedded implementation for testing
type mockDBProvider struct {
    db.Provider
}

// A minimal mock for Rows
type mockRows struct {
    db.Rows
    called int
}

func (m *mockRows) Next() bool {
    m.called++
    return m.called == 1
}

func (m *mockRows) Scan(dest ...any) error {
    // simplified mock
    return nil
}

func (m *mockRows) Close() {}

func (m *mockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &mockRows{}, nil
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    return 1, nil
}

func TestRAGSyncService(t *testing.T) {
    mockDB := &mockDBProvider{}
    svc := NewRAGSyncService(mockDB)

    ctx := context.Background()

    _, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "1", Context: "test"}})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
