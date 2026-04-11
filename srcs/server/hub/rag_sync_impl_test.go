package hub

import (
	"context"
	"encoding/json"
	"testing"
	"time"
    "database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	_ "modernc.org/sqlite"
)

// Minimal mock provider just for testing this logic
type mockDbProvider struct {
	db.Provider
	sqlDB *sql.DB
}

func (m *mockDbProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	res, err := m.sqlDB.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (m *mockDbProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
	rows, err := m.sqlDB.QueryContext(ctx, query, args...)
    if err != nil {
        return nil, err
    }
    return &mockRows{rows: rows}, nil
}

type mockRows struct {
    rows *sql.Rows
}
func (m *mockRows) Next() bool { return m.rows.Next() }
func (m *mockRows) Scan(dest ...any) error { return m.rows.Scan(dest...) }
func (m *mockRows) Close() { m.rows.Close() }
func (m *mockRows) Columns() ([]string, error) { return m.rows.Columns() }
func (m *mockRows) Err() error { return m.rows.Err() }

func (m *mockDbProvider) QueryRow(ctx context.Context, query string, args ...any) db.Row {
	return m.sqlDB.QueryRowContext(ctx, query, args...)
}

func (m *mockDbProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.sqlDB.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	return &mockTx{tx: tx}, nil
}

type mockTx struct {
	db.Tx
	tx *sql.Tx
}

func (m *mockTx) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	res, err := m.tx.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (m *mockTx) Commit(ctx context.Context) error {
	return m.tx.Commit()
}

func (m *mockTx) Rollback(ctx context.Context) error {
	return m.tx.Rollback()
}


func TestRagSyncServiceImpl(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := &mockDbProvider{sqlDB: sqlDB}

	ctx := context.Background()
	_ = telemetry.InitWithMeter(otel.Meter("test"))

    // Prepare table
    _, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
		);
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	service := NewRAGSyncService(provider)

	// Insert pending record
    vectorBytes, _ := json.Marshal([]float32{1.1, 2.2})
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ($1, $2, $3, $4)", "test_mem_1", "ctx1", vectorBytes, "pending")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test_mem_1" {
		t.Errorf("expected test_mem_1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 1.1 {
		t.Errorf("unexpected vector: %v", records[0].Vector)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test_mem_1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test_mem_2",
			Context:    "incoming context",
			Vector:     []float32{3.3, 4.4},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM swarm_memory_embeddings WHERE memory_id = 'test_mem_2'").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("ProcessIncomingSync did not insert record")
	}
}
