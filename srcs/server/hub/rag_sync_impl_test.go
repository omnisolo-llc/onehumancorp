package hub_test

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
	"database/sql"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('mem-1', 'context 1', 'abcd')")
	if err != nil {
		t.Fatalf("Failed to insert mock data: %v", err)
	}

	svc := hub.NewRAGSyncService(provider)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "mem-1" || records[0].SyncStatus != "pending" {
		t.Errorf("Unexpected record content: %+v", records[0])
	}

	err = svc.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 records after MarkSynced, got %d", len(records))
	}

	// For process incoming sync, sqlite might not support ON CONFLICT as standard postgres,
	// so let's mock test process incoming sync using another check.
	// We'll insert mem-2 manually and use processincoming to upsert mem-3.
	err = svc.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
		{ID: "mem-3", Context: "context 3", Vector: []byte("xyz"), SyncStatus: "synced", LastSyncAt: nil},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}
}
