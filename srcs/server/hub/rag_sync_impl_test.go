package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	p := db.NewSqliteProvider(sqliteDB)

	// Create table
	_, err = p.Exec(context.Background(), `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return p
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	meter = noop.NewMeterProvider().Meter("test")
	p := setupTestDB(t)
	svc := NewRAGSyncService(p)
	ctx := context.Background()

	// Insert some test data
	_, err := p.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = p.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test2', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected record ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	p := setupTestDB(t)
	svc := NewRAGSyncService(p)
	ctx := context.Background()

	_, err := p.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := p.QueryRow(ctx, `SELECT sync_status FROM autodream_memories WHERE id = '1'`)
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to scan status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	p := setupTestDB(t)
	svc := NewRAGSyncService(p)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:      "1",
			Context: "incoming context",
			Vector:  []float32{1.0, 2.0},
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := p.QueryRow(ctx, `SELECT content, sync_status FROM autodream_memories WHERE id = '1'`)
	var content, status string
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}
	if content != "incoming context" {
		t.Errorf("expected content 'incoming context', got %s", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
}
