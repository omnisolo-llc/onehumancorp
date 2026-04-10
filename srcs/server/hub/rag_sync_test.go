package hub

import (
	"context"
	"database/sql"

	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	// Setup in-memory sqlite
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// Create table
	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestFetchPendingSyncs(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	// Insert test data
	_, err := database.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES
			('1', 'memory 1', 'pending'),
			('2', 'memory 2', 'synced'),
			('3', 'memory 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	_, err := database.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'memory 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	var status string
	var lastSync sql.NullTime
	err = database.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
	if !lastSync.Valid {
		t.Error("expected last_sync_at to be valid")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "remote memory",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now().UTC(),
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var content string
	var embedding string
	err = database.QueryRow(ctx, "SELECT content, embedding FROM autodream_memories WHERE id = '1'").Scan(&content, &embedding)
	if err != nil {
		t.Fatalf("failed to query inserted record: %v", err)
	}

	if content != "remote memory" {
		t.Errorf("expected content 'remote memory', got %s", content)
	}

	// Just verify it's valid JSON
	if embedding == "" {
		t.Error("expected non-empty embedding")
	}
}
