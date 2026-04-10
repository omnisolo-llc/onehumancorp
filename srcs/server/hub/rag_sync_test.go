package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService_Actual(t *testing.T) {
	ctx := context.Background()

	// Setup SQLite in-memory db
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	// Create table
	createSQL := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	_, err = sqliteDB.Exec(createSQL)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	vec := []float32{1.1, 2.2, 3.3}
	vecBytes, _ := json.Marshal(vec)
	_, err = sqliteDB.Exec(`INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('rec1', 'test content', ?, 'pending')`, string(vecBytes))
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// Normally ProcessIncomingSync uses a Postgres pool (pgxpool.Pool). We can't easily mock it without spinning up a real postgres instance in the test.
	// But we can test FetchPendingSyncs and MarkSynced using the sqlite provider.
	svc := NewRAGSyncService(nil, provider)

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "rec1" {
		t.Errorf("Expected ID rec1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 3 {
		t.Errorf("Expected vector len 3, got %d", len(records[0].Vector))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"rec1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// Verify it was marked synced
	var status string
	err = sqliteDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = 'rec1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status synced, got %s", status)
	}
}

// Test process incoming sync using the mock
func TestProcessIncomingSync_NilPool(t *testing.T) {
	svc := NewRAGSyncService(nil, nil)
	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "rec1"}})
	if err == nil {
		t.Error("Expected error for nil pool")
	}
}
