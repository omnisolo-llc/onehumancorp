package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	d := &db.DB{Provider: provider}

	ctx := context.Background()

	// Setup table according to current state
	_, err = d.Provider.Exec(ctx, "DROP TABLE IF EXISTS autodream_memories")
	if err != nil {
		t.Fatalf("Failed to drop table: %v", err)
	}

	query := `CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = d.Provider.Exec(ctx, query)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return d
}

func TestRAGSyncServiceFlow(t *testing.T) {
	d := setupTestDB(t)
	service := NewDefaultRAGSyncService(d.Provider)
	ctx := context.Background()

	// Insert some test data with vectors
	_, err := d.Provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES (?, ?, ?, ?)", "1", "test 1", "[1.0, 2.0]", SyncStatusPending)
	if err != nil { t.Fatalf("failed to insert data %v", err) }
	_, err = d.Provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "2", "test 2", SyncStatusSynced)
	if err != nil { t.Fatalf("failed to insert data %v", err) }
	_, err = d.Provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "3", "test 3", SyncStatusPending)
	if err != nil { t.Fatalf("failed to insert data %v", err) }

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}
	if pending[0].ID != "1" && pending[1].ID != "1" {
		t.Errorf("Missing record 1")
	}

	// Check vector parsing
	for _, p := range pending {
		if p.ID == "1" {
			if len(p.Vector) != 2 || p.Vector[0] != 1.0 {
				t.Errorf("Vector not parsed correctly: %v", p.Vector)
			}
		}
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending2) != 1 {
		t.Fatalf("Expected 1 pending record after marking 1 synced, got %d", len(pending2))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "test 4", Vector: []float32{3.0, 4.0}, SyncStatus: SyncStatusPending},
		{ID: "2", Context: "test 2 updated", Vector: []float32{5.0}, SyncStatus: SyncStatusPending}, // Test upsert
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = d.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("QueryRow failed: %v", err)
	}
	if count != 3 { // 2 originally synced + 1 synced + 1 new synced via ProcessIncomingSync (upserts make sync_status='synced')
		t.Fatalf("Expected 3 synced records, got %d", count)
	}
}
