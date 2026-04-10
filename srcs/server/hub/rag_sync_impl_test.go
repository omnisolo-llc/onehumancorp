package hub

import (
    "context"
    "database/sql"
    "errors"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

// MockProvider is a simple mock of the db.Provider for testing purposes
type MockProvider struct {
    db.Provider // Embed the interface
    QueryFunc   func(ctx context.Context, query string, args ...any) (db.Rows, error)
    ExecFunc    func(ctx context.Context, query string, args ...any) (int64, error)
    BeginFunc   func(ctx context.Context) (db.Tx, error)
}

func (m *MockProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
    if m.QueryFunc != nil {
        return m.QueryFunc(ctx, query, args...)
    }
    return nil, errors.New("Query not implemented")
}

func (m *MockProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    if m.ExecFunc != nil {
        return m.ExecFunc(ctx, query, args...)
    }
    return 0, errors.New("Exec not implemented")
}

func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
    if m.BeginFunc != nil {
        return m.BeginFunc(ctx)
    }
    return nil, errors.New("Begin not implemented")
}

// MockTx is a simple mock of db.Tx
type MockTx struct {
    db.Tx
    ExecFunc     func(ctx context.Context, query string, args ...any) (int64, error)
    CommitFunc   func(ctx context.Context) error
    RollbackFunc func(ctx context.Context) error
}

func (m *MockTx) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    if m.ExecFunc != nil {
        return m.ExecFunc(ctx, query, args...)
    }
    return 0, errors.New("Tx Exec not implemented")
}

func (m *MockTx) Commit(ctx context.Context) error {
    if m.CommitFunc != nil {
        return m.CommitFunc(ctx)
    }
    return nil
}

func (m *MockTx) Rollback(ctx context.Context) error {
    if m.RollbackFunc != nil {
        return m.RollbackFunc(ctx)
    }
    return nil
}

// MockRows is a simple mock of db.Rows
type MockRows struct {
    db.Rows
    NextFunc  func() bool
    ScanFunc  func(dest ...any) error
    CloseFunc func()
    ErrFunc   func() error
}

func (m *MockRows) Next() bool {
    if m.NextFunc != nil {
        return m.NextFunc()
    }
    return false
}

func (m *MockRows) Scan(dest ...any) error {
    if m.ScanFunc != nil {
        return m.ScanFunc(dest...)
    }
    return nil
}

func (m *MockRows) Close() {
    if m.CloseFunc != nil {
        m.CloseFunc()
    }
}

func (m *MockRows) Err() error {
    if m.ErrFunc != nil {
        return m.ErrFunc()
    }
    return nil
}

func TestFetchPendingSyncsImpl(t *testing.T) {
    ctx := context.Background()
    now := time.Now()

    // Test successful fetch
    provider := &MockProvider{
        QueryFunc: func(ctx context.Context, query string, args ...any) (db.Rows, error) {
            rowsReturned := 0
            return &MockRows{
                NextFunc: func() bool {
                    rowsReturned++
                    return rowsReturned <= 1
                },
                ScanFunc: func(dest ...any) error {
                    *dest[0].(*string) = "1"
                    *dest[1].(*string) = "context"

                    *dest[2].(*sql.NullString) = sql.NullString{String: "[0.1, 0.2]", Valid: true}
                    *dest[3].(*sql.NullString) = sql.NullString{String: string(SyncStatusPending), Valid: true}
                    *dest[4].(*sql.NullTime) = sql.NullTime{Time: now, Valid: true}
                    return nil
                },
                CloseFunc: func() {},
                ErrFunc: func() error { return nil },
            }, nil
        },
    }

    svc := NewRAGSyncService(provider)
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "1" {
        t.Errorf("expected ID 1, got %s", records[0].ID)
    }
    if len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 {
        t.Errorf("expected Vector to be populated, got %+v", records[0].Vector)
    }
    if records[0].LastSyncAt == nil || !records[0].LastSyncAt.Equal(now) {
        t.Errorf("expected LastSyncAt to be populated correctly")
    }

    // Test query error
    providerErr := &MockProvider{
        QueryFunc: func(ctx context.Context, query string, args ...any) (db.Rows, error) {
            return nil, errors.New("db error")
        },
    }
    svcErr := NewRAGSyncService(providerErr)
    _, err = svcErr.FetchPendingSyncs(ctx, 10)
    if err == nil {
        t.Fatal("expected error, got nil")
    }
}

func TestMarkSyncedImpl(t *testing.T) {
    ctx := context.Background()

    provider := &MockProvider{
        BeginFunc: func(ctx context.Context) (db.Tx, error) {
            return &MockTx{
                ExecFunc: func(ctx context.Context, query string, args ...any) (int64, error) {
                    return 1, nil
                },
                CommitFunc: func(ctx context.Context) error {
                    return nil
                },
                RollbackFunc: func(ctx context.Context) error {
                    return nil
                },
            }, nil
        },
    }

    svc := NewRAGSyncService(provider)
    err := svc.MarkSynced(ctx, []string{"1", "2"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    // Test with error
    providerErr := &MockProvider{
        BeginFunc: func(ctx context.Context) (db.Tx, error) {
            return nil, errors.New("db error")
        },
    }
    svcErr := NewRAGSyncService(providerErr)
    err = svcErr.MarkSynced(ctx, []string{"1"})
    if err == nil {
        t.Fatal("expected error, got nil")
    }
}

func TestProcessIncomingSyncImpl(t *testing.T) {
    ctx := context.Background()
    now := time.Now()

    provider := &MockProvider{
        BeginFunc: func(ctx context.Context) (db.Tx, error) {
            return &MockTx{
                ExecFunc: func(ctx context.Context, query string, args ...any) (int64, error) {
                    return 1, nil
                },
                CommitFunc: func(ctx context.Context) error {
                    return nil
                },
                RollbackFunc: func(ctx context.Context) error {
                    return nil
                },
            }, nil
        },
    }

    svc := NewRAGSyncService(provider)
    err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "1", Context: "test", SyncStatus: SyncStatusSynced, Vector: []float32{0.1, 0.2}, LastSyncAt: &now},
    })
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    // Test with error
    providerErr := &MockProvider{
         BeginFunc: func(ctx context.Context) (db.Tx, error) {
            return nil, errors.New("db error")
        },
    }
    svcErr := NewRAGSyncService(providerErr)
    err = svcErr.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "1", Context: "test", SyncStatus: SyncStatusSynced},
    })
    if err == nil {
        t.Fatal("expected error, got nil")
    }
}
