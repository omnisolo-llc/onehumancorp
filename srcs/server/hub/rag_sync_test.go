package hub

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type MockProvider struct {
    ExecCalled bool
    Queries []string
}

func (m *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.ExecCalled = true
    m.Queries = append(m.Queries, sql)
    return 1, nil
}
func (m *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &MockRows{}, nil
}
func (m *MockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
    return nil
}
func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
    return &MockTx{provider: m}, nil
}
func (m *MockProvider) Close() {}
func (m *MockProvider) IsSQLite() bool { return true }
func (m *MockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) { return nil, nil }

type MockRows struct {
    count int
}
func (m *MockRows) Next() bool { m.count++; return m.count <= 1 }
func (m *MockRows) Scan(dest ...any) error {
    id := dest[0].(*string)
    *id = "1"
    ctxStr := dest[1].(*string)
    *ctxStr = "memory1"
    syncStatus := dest[2].(*SyncStatus)
    *syncStatus = SyncStatusPending
    lastSyncAt := dest[3].(**time.Time)
    now := time.Now()
    *lastSyncAt = &now
    return nil
}
func (m *MockRows) Close() {}
func (m *MockRows) Columns() ([]string, error) { return nil, nil }
func (m *MockRows) Err() error { return nil }

type MockTx struct {
    provider *MockProvider
}
func (m *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    return m.provider.Exec(ctx, sql, arguments...)
}
func (m *MockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *MockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *MockTx) Commit(ctx context.Context) error { return nil }
func (m *MockTx) Rollback(ctx context.Context) error { return nil }

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
    p := &MockProvider{}
    svc := NewRAGSyncService(p)
    records, err := svc.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "1" {
        t.Fatalf("expected record ID '1', got '%s'", records[0].ID)
    }
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
    p := &MockProvider{}
    svc := NewRAGSyncService(p)
    err := svc.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if !p.ExecCalled {
        t.Fatalf("expected Exec to be called")
    }
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
    p := &MockProvider{}
    svc := NewRAGSyncService(p)
    records := []RAGSyncRecord{
        {ID: "1", Context: "incoming memory", Vector: []float32{0.1, 0.2}},
    }
    err := svc.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if !p.ExecCalled {
        t.Fatalf("expected Exec to be called")
    }
}
