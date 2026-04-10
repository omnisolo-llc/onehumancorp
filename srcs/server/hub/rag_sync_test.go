package hub

import (
    "context"
    "database/sql"
    "encoding/json"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

// sqliteProvider is a quick mock wrapper to provide a real sqlite instance
// since we can't import the test_provider from db package directly
type sqliteProvider struct {
    db *sql.DB
}

func (p *sqliteProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    res, err := p.db.ExecContext(ctx, sql, arguments...)
    if err != nil {
        return 0, err
    }
    return res.RowsAffected()
}

type mockRows struct {
    *sql.Rows
}
func (m *mockRows) Close() {
    _ = m.Rows.Close()
}
func (m *mockRows) Columns() ([]string, error) {
    return m.Rows.Columns()
}
func (p *sqliteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    r, err := p.db.QueryContext(ctx, sql, optionsAndArgs...)
    if err != nil {
        return nil, err
    }
    return &mockRows{r}, nil
}

type mockRow struct {
    *sql.Row
}
func (p *sqliteProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
    return &mockRow{p.db.QueryRowContext(ctx, sql, optionsAndArgs...)}
}

func (p *sqliteProvider) Begin(ctx context.Context) (db.Tx, error) {
    return nil, nil // unused in this test
}

func (p *sqliteProvider) Close() {
    p.db.Close()
}

func (p *sqliteProvider) IsSQLite() bool {
    return true
}

func (p *sqliteProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
    return nil, nil // unused
}

func TestRAGSyncService(t *testing.T) {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    provider := &sqliteProvider{db: sqlDB}
    defer provider.Close()

    ctx := context.Background()

    // Initialize DB to match exactly the trace table + new sync columns
    _, err = provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMPTZ NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    svc := NewRAGSyncService(provider)

    // Insert pending record
    vectorData := []float32{0.1, 0.2, 0.3}
    vectorBytes, _ := json.Marshal(vectorData)
    _, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('123', 'test context', $1)`, vectorBytes)
    if err != nil {
        t.Fatalf("failed to insert test record: %v", err)
    }

    // Test FetchPendingSyncs
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if len(records[0].Vector) != 3 {
         t.Fatalf("expected vector of length 3, got %d", len(records[0].Vector))
    }

    // Test MarkSynced
    err = svc.MarkSynced(ctx, []string{"123"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    records, err = svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 pending records, got %d", len(records))
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {ID: "456", Context: "remote context", Vector: []float32{0.4, 0.5}, SyncStatus: SyncStatusSynced},
    }
    err = svc.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
