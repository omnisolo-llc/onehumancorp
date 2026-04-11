package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockTx struct {
    execFunc     func(ctx context.Context, sql string, arguments ...any) (int64, error)
    queryFunc    func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
    commitFunc   func(ctx context.Context) error
    rollbackFunc func(ctx context.Context) error
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    if m.execFunc != nil {
        return m.execFunc(ctx, sql, arguments...)
    }
    return 0, nil
}

func (m *mockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    if m.queryFunc != nil {
        return m.queryFunc(ctx, sql, optionsAndArgs...)
    }
    return nil, nil
}

func (m *mockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
    return nil
}

func (m *mockTx) Commit(ctx context.Context) error {
    if m.commitFunc != nil {
        return m.commitFunc(ctx)
    }
    return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
    if m.rollbackFunc != nil {
        return m.rollbackFunc(ctx)
    }
    return nil
}

type mockProvider struct {
    execFunc       func(ctx context.Context, sql string, arguments ...any) (int64, error)
    queryFunc      func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
    beginFunc      func(ctx context.Context) (db.Tx, error)
    isSQLiteFunc   func() bool
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    if m.execFunc != nil {
        return m.execFunc(ctx, sql, arguments...)
    }
    return 0, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    if m.queryFunc != nil {
        return m.queryFunc(ctx, sql, optionsAndArgs...)
    }
    return nil, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
    return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
    if m.beginFunc != nil {
        return m.beginFunc(ctx)
    }
    return &mockTx{}, nil
}

func (m *mockProvider) Close() {}

func (m *mockProvider) IsSQLite() bool {
    if m.isSQLiteFunc != nil {
        return m.isSQLiteFunc()
    }
    return false
}

func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
    return nil, nil
}

type mockRows struct {
    nextFunc  func() bool
    scanFunc  func(dest ...any) error
    closeFunc func()
    errFunc   func() error
}

func (m *mockRows) Next() bool {
    if m.nextFunc != nil {
        return m.nextFunc()
    }
    return false
}

func (m *mockRows) Scan(dest ...any) error {
    if m.scanFunc != nil {
        return m.scanFunc(dest...)
    }
    return nil
}

func (m *mockRows) Close() {
    if m.closeFunc != nil {
        m.closeFunc()
    }
}

func (m *mockRows) Columns() ([]string, error) {
    return nil, nil
}

func (m *mockRows) Err() error {
    if m.errFunc != nil {
        return m.errFunc()
    }
    return nil
}

func TestDBAGSyncService_FetchPendingSyncs(t *testing.T) {
    var queryCalled bool
    mockRows := &mockRows{
        nextFunc: func() bool {
            if !queryCalled {
                queryCalled = true
                return true
            }
            return false
        },
        scanFunc: func(dest ...any) error {
            id := dest[0].(*string)
            *id = "123"
            return nil
        },
    }

    mockTx := &mockTx{
        queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
            return mockRows, nil
        },
    }

    mockProv := &mockProvider{
        beginFunc: func(ctx context.Context) (db.Tx, error) {
            return mockTx, nil
        },
    }

    svc := NewDBAGSyncService(mockProv)
    records, err := svc.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "123" {
        t.Errorf("expected ID '123', got %s", records[0].ID)
    }
}

func TestDBAGSyncService_MarkSynced(t *testing.T) {
    var execCalled int
    mockProv := &mockProvider{
        execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
            execCalled++
            return 1, nil
        },
    }

    svc := NewDBAGSyncService(mockProv)
    err := svc.MarkSynced(context.Background(), []string{"1", "2"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if execCalled != 2 {
        t.Errorf("expected 2 exec calls, got %d", execCalled)
    }
}

func TestDBAGSyncService_ProcessIncomingSync(t *testing.T) {
    var execCalled int
    mockProv := &mockProvider{
        execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
            execCalled++
            return 1, nil
        },
    }

    svc := NewDBAGSyncService(mockProv)
    records := []RAGSyncRecord{
        {ID: "1", Context: "test", Vector: []byte{1}},
    }
    err := svc.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if execCalled != 1 {
        t.Errorf("expected 1 exec call, got %d", execCalled)
    }
}
