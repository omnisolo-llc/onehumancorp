package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test db: %v", err)
	}

	_, err = sqliteDB.Exec(`DROP TABLE IF EXISTS autodream_memories`)
	if err != nil {
		t.Fatalf("Failed to drop table: %v", err)
	}

	// Create table matching the schema we expect
	schema := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	_, err = sqliteDB.Exec(schema)
	if err != nil {
		t.Fatalf("Failed to create test schema: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('test-id-1', 'context 1', '[0.1, 0.2]', 'pending')`)
	provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('test-id-2', 'context 2', 'null', 'synced')`)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	if records[0].ID != "test-id-1" {
		t.Errorf("Expected id test-id-1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 {
		t.Errorf("Vector parsing failed: %v", records[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-id-1', 'context 1', 'pending')`)

	err := service.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	err = provider.QueryRow(ctx, `SELECT sync_status FROM autodream_memories WHERE id = 'test-id-1'`).Scan(&status)

	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status synced, got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "cloud-id-1",
			Context:    "cloud context",
			Vector:     []float32{0.5, 0.5},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var content, vector string
	err = provider.QueryRow(ctx, `SELECT content, embedding FROM autodream_memories WHERE id = 'cloud-id-1'`).Scan(&content, &vector)

	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	if content != "cloud context" {
		t.Errorf("Expected 'cloud context', got %s", content)
	}
	if vector != "[0.5,0.5]" {
		t.Errorf("Expected '[0.5,0.5]', got %s", vector)
	}
}
