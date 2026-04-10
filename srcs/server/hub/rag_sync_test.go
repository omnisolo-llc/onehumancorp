package hub

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type MockRows struct {
    records []RAGSyncRecord
    pos     int
}

func (r *MockRows) Next() bool {
    r.pos++
    return r.pos <= len(r.records)
}

func (r *MockRows) Scan(dest ...any) error {
    rec := r.records[r.pos-1]
    *dest[0].(*string) = rec.ID
    *dest[1].(*string) = rec.Context
    *dest[2].(*string) = string(rec.SyncStatus)

    if !rec.LastSyncAt.IsZero() {
         *dest[3].(**time.Time) = &rec.LastSyncAt
    } else {
         *dest[3].(**time.Time) = nil
    }
    return nil
}

func (r *MockRows) Close() {}
func (r *MockRows) Columns() ([]string, error) { return nil, nil }
func (r *MockRows) Err() error { return nil }

type MockTx struct{}
func (t *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (t *MockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (t *MockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (t *MockTx) Commit(ctx context.Context) error { return nil }
func (t *MockTx) Rollback(ctx context.Context) error { return nil }

type MockProvider struct{}
func (p *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (p *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    return &MockRows{
        records: []RAGSyncRecord{
            {ID: "test-1", Context: "ctx-1", SyncStatus: SyncStatusPending},
        },
    }, nil
}
func (p *MockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (p *MockProvider) Begin(ctx context.Context) (db.Tx, error) { return &MockTx{}, nil }
func (p *MockProvider) Close() {}
func (p *MockProvider) IsSQLite() bool { return false }
func (p *MockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) { return nil, nil }

func TestDefaultRAGSyncService(t *testing.T) {
    provider := &MockProvider{}
    service := NewRAGSyncService(provider)
    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("Expected 1 record, got %d", len(records))
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"test-1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Test ProcessIncomingSync
    err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "test-2", Context: "ctx-2"},
    })
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
