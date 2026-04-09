package hub

import (
	"context"
	"testing"
	"database/sql"
	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite memory db: %v", err)
	}

	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)

	_, err := provider.Exec(context.Background(), `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert mock data: %v", err)
	}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].Context != "test context" {
		t.Errorf("Expected context 'test context', got '%s'", records[0].Context)
	}
}
