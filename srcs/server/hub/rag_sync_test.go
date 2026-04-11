package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
    isSQLite bool
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTx{}, nil
}
func (m *mockProvider) Close() {}
func (m *mockProvider) IsSQLite() bool { return m.isSQLite }
func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) { return nil, nil }

type mockTx struct {}
func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (m *mockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &mockRows{count: 1}, nil
}
func (m *mockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *mockTx) Commit(ctx context.Context) error { return nil }
func (m *mockTx) Rollback(ctx context.Context) error { return nil }

type mockRows struct {
    count int
    idx   int
}
func (m *mockRows) Next() bool {
    if m.idx < m.count {
        m.idx++
        return true
    }
    return false
}
func (m *mockRows) Scan(dest ...any) error {
    *dest[0].(*string) = "1"
    *dest[1].(*string) = "test context"
    *dest[2].(*[]byte) = []byte{1, 2, 3}
    *dest[3].(*SyncStatus) = SyncStatusPending
    // dest[4] is **time.Time which is nil for lastSyncAt initially
    return nil
}
func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

func TestRAGSyncService_Flow(t *testing.T) {
    ctx := context.Background()

    // Test Postgres Flow
    pgProvider := &mockProvider{isSQLite: false}
    svc := NewRAGSyncService(pgProvider)

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "context 2", Vector: []byte{4, 5, 6}},
    })
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    // Test SQLite Flow
    sqliteProvider := &mockProvider{isSQLite: true}
    svcSQLite := NewRAGSyncService(sqliteProvider)

    recordsSq, errSq := svcSQLite.FetchPendingSyncs(ctx, 10)
    if errSq != nil {
        t.Fatalf("expected no error, got %v", errSq)
    }
    if len(recordsSq) != 1 {
        t.Fatalf("expected 1 record, got %d", len(recordsSq))
    }
}
