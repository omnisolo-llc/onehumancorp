package db

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t *testing.T) Provider {
	t.Helper()
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Hardcoded initialization for testing
	queries := []string{
		`CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system',
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP
		);`,
	}
	for _, q := range queries {
		if _, err := db.Exec(q); err != nil {
			t.Fatalf("failed to initialize test tables: %v", err)
		}
	}
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := db.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		db.Close()
	})

	return NewSqliteProvider(db)
}
