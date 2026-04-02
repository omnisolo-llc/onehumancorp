package sync

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestProcessSyncTick(t *testing.T) {
	os.Setenv("GO_ENV", "test")
	defer os.Unsetenv("GO_ENV")
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}

	provider := dbWrapper.Provider

	// Explicitly define the schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE embedding_cache (
			content_hash TEXT PRIMARY KEY,
			embedding TEXT NOT NULL,
			synced_to_cloud BOOLEAN DEFAULT false,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES (?, ?, ?)", "hash1", "[0.1]", false)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES (?, ?, ?)", "hash2", "[0.2]", false)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES (?, ?, ?)", "hash3", "[0.3]", true) // Already synced
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Verify pre-sync state
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&count)
	if err != nil || count != 2 {
		t.Fatalf("expected 2 unsynced rows, got %d, err: %v", count, err)
	}

	worker := NewAutoDreamSyncWorker(provider)
	worker.ProcessSyncTick(ctx)

	// Verify post-sync state
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&count)
	if err != nil || count != 0 {
		t.Fatalf("expected 0 unsynced rows after sync, got %d, err: %v", count, err)
	}
}
