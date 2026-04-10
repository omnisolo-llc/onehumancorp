package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

// MockProvider implements db.Provider for testing
type MockProvider struct {
    db.Provider
    execArgs [][]any
}

func (m *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &MockRows{}, nil
}
func (m *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.execArgs = append(m.execArgs, arguments)
    return 1, nil
}

type MockRows struct {
    db.Rows
    count int
}
func (m *MockRows) Next() bool {
    m.count++
    return m.count <= 1
}
func (m *MockRows) Scan(dest ...any) error {
    return nil
}
func (m *MockRows) Close() {}

func TestRAGSyncService(t *testing.T) {
    provider := &MockProvider{}
    service := NewRAGSyncService(provider)

    ctx := context.Background()
    _, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = service.MarkSynced(ctx, []string{"id1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "id1", Context: "test", Vector: []float32{1.0, 2.0}}})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    if len(provider.execArgs) < 2 {
        t.Fatalf("expected at least 2 exec calls")
    }

    foundVector := false
    for _, arg := range provider.execArgs[1] {
        if arg == "[1.000000,2.000000]" {
            foundVector = true
            break
        }
    }
    if !foundVector {
        t.Fatalf("did not find vector string in arguments: %v", provider.execArgs[1])
    }
}
