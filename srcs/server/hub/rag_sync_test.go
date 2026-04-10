package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSyncDaemon(t *testing.T) {
	// Use in-memory SQLite for testing to verify queries execute correctly
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}
	defer provider.Close()

	daemon := NewSyncDaemon(provider)

	// Since swarm_memory_embeddings does not exist in memory natively,
	// the provider test setup would usually run migrations. If we assume migrations run,
	// or we can test basic structural queries manually if we create the table.
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	// Insert test data
	vec := []float32{1.0, 2.0, 3.0}
	vecBytes := encodeVector(vec)
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('1', 'test_context', $1, 'pending')
	`, vecBytes)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := daemon.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	} else {
	    if records[0].ID != "1" {
		    t.Errorf("expected record ID 1, got %s", records[0].ID)
	    }
	    if len(records[0].Vector) != 3 || records[0].Vector[0] != 1.0 {
	        t.Errorf("vector data incorrect")
	    }
	}

	// Test MarkSynced
	err = daemon.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	// Verify MarkSynced
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "2", Context: "new_context", Vector: []float32{4.0, 5.0}},
	}
	err = daemon.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	var vBytes []byte
	err = provider.QueryRow(ctx, "SELECT COUNT(*), vector_embedding FROM swarm_memory_embeddings WHERE memory_id = '2' AND sync_status = 'synced'").Scan(&count, &vBytes)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record for incoming sync, got %d", count)
	}
	v := decodeVector(vBytes)
	if len(v) != 2 || v[0] != 4.0 {
	    t.Errorf("vector data incoming incorrect")
	}
}
