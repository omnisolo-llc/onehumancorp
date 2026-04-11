package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	_, err = dbConn.Exec(`CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "context1", Vector: []byte("vec1")},
	}
	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert pending directly
	_, err = dbConn.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('mem2', 'context2', 'vec2', 'pending')")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem2" {
		t.Fatalf("unexpected pending records: %+v", pending)
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending2, _ := service.FetchPendingSyncs(ctx, 10)
	if len(pending2) != 0 {
		t.Fatalf("expected 0 pending, got %d", len(pending2))
	}
}
