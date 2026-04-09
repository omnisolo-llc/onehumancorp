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
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}

	// Set up schema
	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	return provider
}

func TestDB_RAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	svc := &DB_RAGSyncService{Provider: provider}
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'hello', '[0.1, 0.2]', 'pending')")
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 || records[0].ID != "1" {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 {
		t.Fatalf("Vector not properly parsed")
	}

	// Mark synced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// Verify no pending
	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Process Incoming (UPSERT logic test)
	newRec := RAGSyncRecord{ID: "2", Context: "world", Vector: []float32{0.5, 0.6}, SyncStatus: "synced", LastSyncAt: time.Now()}
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{newRec})
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}
}
