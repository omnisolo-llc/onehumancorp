package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	provider := db.NewSqliteProvider(dbConn)

	// Create schema
	_, err = provider.Exec(context.Background(), `CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert pending
	_, err = provider.Exec(context.Background(), `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	// Fetch pending
	pending, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("Fetch failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "1" {
		t.Fatalf("Expected 1 pending record, got %v", pending)
	}

	// Mark synced
	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch again
	pending2, _ := service.FetchPendingSyncs(context.Background(), 10)
	if len(pending2) != 0 {
		t.Fatalf("Expected 0 pending records after sync")
	}

	// Process incoming
	rec := RAGSyncRecord{ID: "2", Context: "ctx2", SyncStatus: SyncStatusSynced}
	err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{rec})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
}
