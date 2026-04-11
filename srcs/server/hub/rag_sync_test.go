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
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()

	_, err = provider.Exec(ctx, "DROP TABLE IF EXISTS autodream_memories")
	if err != nil {
		t.Fatalf("failed to drop table: %v", err)
	}

	createTableQuery := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);`

	_, err = provider.Exec(ctx, createTableQuery)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'context 1', '[0.1, 0.2]', 'pending'),
		       ('2', 'context 2', '[0.3, 0.4]', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", records[0].ID)
	}

	if len(records[0].Vector) != 2 {
		t.Errorf("expected vector of length 2, got %d", len(records[0].Vector))
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'context 1', '[0.1, 0.2]', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	rows, err := provider.Query(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query after update: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("expected row to exist")
	}

	var status string
	var lastSync sql.NullTime
	if err := rows.Scan(&status, &lastSync); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	if !lastSync.Valid {
		t.Error("expected last_sync_at to be valid")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert initial data to test conflict update
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'old context', '[0.0, 0.0]', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "new context",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "2",
			Context:    "new record",
			Vector:     nil, // Test nil vector
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify update for ID 1
	var content string
	var vector string
	rows, err := provider.Query(ctx, "SELECT content, embedding FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query ID 1: %v", err)
	}
	defer rows.Close()
	rows.Next()
	if err := rows.Scan(&content, &vector); err != nil {
		t.Fatalf("failed to scan ID 1: %v", err)
	}

	if content != "new context" {
		t.Errorf("expected 'new context', got '%s'", content)
	}

	if vector != "[0.5,0.6]" {
		t.Errorf("expected '[0.5,0.6]', got '%s'", vector)
	}
}
