package hub

import (
	"context"
	"database/sql"
	"testing"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	return provider
}

func TestRAGSyncService_Standalone_FetchPending(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)

	// Setup schema with vector_embedding
	_, err := provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
            vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	svc := NewRAGSyncService(provider)
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" || records[0].Context != "ctx1" {
		t.Fatalf("unexpected record: %+v", records[0])
	}
}

func TestRAGSyncService_Standalone_MarkSynced(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)

	// Setup schema
	_, err := provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
            vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	svc := NewRAGSyncService(provider)
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify
	rows, err := provider.Query(ctx, `SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'`)
	if err != nil {
		t.Fatalf("failed to query: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected 1 row")
	}

	var status string
	if err := rows.Scan(&status); err != nil {
		t.Fatalf("failed to scan: %v", err)
	}

	if status != "synced" {
		t.Fatalf("expected 'synced', got '%s'", status)
	}
}
