package hub

import (
    "context"
    "testing"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockRows struct {
    db.Rows
    records []RAGSyncRecord
    index int
}

func (m *mockRows) Next() bool {
    m.index++
    return m.index <= len(m.records)
}

func (m *mockRows) Scan(dest ...any) error {
    rec := m.records[m.index-1]
    *dest[0].(*string) = rec.MemoryID
    *dest[1].(*string) = rec.Context
    *dest[2].(*[]byte) = rec.Vector
    *dest[3].(**string) = rec.SourcePlugin
    *dest[4].(*time.Time) = rec.CreatedAt
    *dest[5].(*SyncStatus) = rec.SyncStatus
    *dest[6].(**time.Time) = rec.LastSyncAt
    return nil
}

func (m *mockRows) Close() {}

func (m *mockRows) Err() error { return nil }

type mockProvider struct {
    db.Provider
    IsSQLiteVal bool
    execQuery string
    execArgs []any
    queryQuery string
    queryArgs []any
    mockRecords []RAGSyncRecord
}

func (m *mockProvider) IsSQLite() bool {
    return m.IsSQLiteVal
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.execQuery = sql
    m.execArgs = arguments
    return 1, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    m.queryQuery = sql
    m.queryArgs = optionsAndArgs
    return &mockRows{records: m.mockRecords}, nil
}

func TestRAGSyncService(t *testing.T) {
    mockProv := &mockProvider{IsSQLiteVal: false}
    mockSvc := NewRAGSyncService(mockProv)

    ctx := context.Background()

    now := time.Now()
    plugin := "plugin-1"
    record := RAGSyncRecord{
        MemoryID:     "test-1",
        Context:      "some context",
        Vector:       []byte{1, 2, 3},
        SourcePlugin: &plugin,
        CreatedAt:    now,
        SyncStatus:   SyncStatusPending,
        LastSyncAt:   nil,
    }

    err := mockSvc.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    mockProv.mockRecords = []RAGSyncRecord{record}

    records, err := mockSvc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(records) != 1 {
        t.Errorf("Expected 1 record, got %d", len(records))
    }

    err = mockSvc.MarkSynced(ctx, []string{"test-1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
}
