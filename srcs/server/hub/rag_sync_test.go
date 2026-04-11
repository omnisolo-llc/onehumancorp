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
		t.Fatalf("Failed to open sqlite: %v", err)
	}

	provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

	ctx := context.Background()

	// Setup table
	_, err = provider.Exec(ctx, `DROP TABLE IF EXISTS autodream_memories`)
	if err != nil {
		t.Fatalf("Failed to drop table: %v", err)
	}

	_, err = provider.Exec(ctx, `CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT,
		embedding TEXT,
		sync_status TEXT,
		last_sync_timestamp TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	service := NewRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("Expected ID '1', got '%s'", records[0].ID)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("Expected status 'pending', got '%s'", records[0].SyncStatus)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	service := NewRAGSyncService(provider)

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify sync_status
	rows, err := provider.Query(ctx, `SELECT sync_status FROM autodream_memories WHERE id = '1'`)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	if rows.Next() {
		var status string
		err = rows.Scan(&status)
		if err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
		if status != "synced" {
			t.Errorf("Expected status 'synced', got '%s'", status)
		}
	} else {
		t.Fatalf("Record not found after update")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	service := NewRAGSyncService(provider)

	record := RAGSyncRecord{
		ID:         "2",
		Context:    "incoming context",
		SyncStatus: SyncStatusSynced,
		LastSyncAt: time.Now(),
	}

	err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insert
	rows, err := provider.Query(ctx, `SELECT id, content, sync_status FROM autodream_memories WHERE id = '2'`)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	if rows.Next() {
		var id, content, status string
		err = rows.Scan(&id, &content, &status)
		if err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
		if id != "2" || content != "incoming context" || status != "synced" {
			t.Errorf("Unexpected values: %s, %s, %s", id, content, status)
		}
	} else {
		t.Fatalf("Record not found after insert")
	}
}
