package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, *sql.DB) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}

	// Create autodream_memories table
	createTableSQL := `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	_, err = sqlDB.Exec(createTableSQL)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	return provider, sqlDB
}

func TestFetchPendingSyncs(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service, err := NewSQLRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	// Insert some test data
	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'context 1', 'pending')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'context 2', 'synced')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("Expected 1 pending record, got %d", len(records))
	}
	if len(records) > 0 && records[0].ID != "1" {
		t.Errorf("Expected record ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service, err := NewSQLRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'context 1', 'pending')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	rows, err := provider.Query(ctx, `SELECT sync_status FROM autodream_memories WHERE id = '1'`)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	var status string
	if rows.Next() {
		if err := rows.Scan(&status); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
	}

	if status != "synced" {
		t.Errorf("Expected status synced, got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service, err := NewSQLRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "synced context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	rows, err := provider.Query(ctx, `SELECT content, sync_status FROM autodream_memories WHERE id = '1'`)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	var content, status string
	if rows.Next() {
		if err := rows.Scan(&content, &status); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
	}

	if content != "synced context" {
		t.Errorf("Expected content 'synced context', got %s", content)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got %s", status)
	}
}
