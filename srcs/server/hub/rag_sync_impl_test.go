package hub

import (
	"context"
	"os"
	"testing"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, func()) {
	tmpFile, err := os.CreateTemp("", "test_hub_*.db")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	cleanup := func() {
		provider.Close()
		os.Remove(tmpFile.Name())
	}

	return provider, cleanup
}

func TestDBRAGSyncService(t *testing.T) {
	provider, cleanup := setupTestDB(t)
	defer cleanup()

	svc := NewDBRAGSyncService(provider)
	ctx := context.Background()

	// Seed some data
	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES (?, ?, ?)`, "m1", "Memory 1", []byte{1, 2, 3})
	if err != nil {
		t.Fatalf("failed to seed: %v", err)
	}
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES (?, ?, ?)`, "m2", "Memory 2", []byte{4, 5, 6})
	if err != nil {
		t.Fatalf("failed to seed: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "m2" {
		t.Fatalf("expected 1 pending (m2), got %v", pending)
	}

	// Test process incoming
	incoming := []RAGSyncRecord{
		{ID: "m3", Context: "Memory 3", Vector: []byte{7, 8, 9}},
		{ID: "m2", Context: "Memory 2 updated", Vector: []byte{10, 11, 12}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Check final state
	var count int
	row := provider.QueryRow(ctx, `SELECT COUNT(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'`)
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to scan count: %v", err)
	}
	if count != 3 {
		t.Fatalf("expected 3 synced records, got %d", count)
	}
}
