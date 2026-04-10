package hub

import (
	"context"
	"database/sql"
	"testing"
    _ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize test database directly for isolated test
    sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}
	defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

	// Create tables
	conn := provider

	_, err = conn.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "id1", Context: "context1", Vector: []byte("vector1")},
		{ID: "id2", Context: "context2", Vector: []byte("vector2")},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert pending records
	_, err = conn.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('id3', 'context3', 'vector3', 'pending'),
		       ('id4', 'context4', 'vector4', 'pending')
	`)
	if err != nil {
		t.Fatalf("Failed to insert pending records: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(records))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"id3", "id4"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}
}
