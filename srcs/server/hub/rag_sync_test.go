package hub

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBProvider struct {
    db.Provider
    execCalled bool
}

func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTx{provider: m}, nil
}

type mockTx struct {
    db.Tx
    provider *mockDBProvider
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.provider.execCalled = true
    return 1, nil
}

func (m *mockTx) Commit(ctx context.Context) error {
    return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
    return nil
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
    mockDB := &mockDBProvider{}
    service := NewRAGSyncService(mockDB)

    ctx := context.Background()
    err := service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    if !mockDB.execCalled {
        t.Errorf("expected exec to be called on mock tx")
    }
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
    mockDB := &mockDBProvider{}
    service := NewRAGSyncService(mockDB)

    ctx := context.Background()
    err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "test2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    })

    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    if !mockDB.execCalled {
        t.Errorf("expected exec to be called on mock tx")
    }
}
