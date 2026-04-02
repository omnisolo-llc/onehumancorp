package sync

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // modernc sqlite driver required by memory config
)

func TestAutoDreamSyncEngine_ProcessForecastTick(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")

	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to init db: %v", err)
	}
	provider := dbWrapper.Provider

	// Set up schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE embedding_cache (
			content_hash TEXT PRIMARY KEY,
			embedding BLOB NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT false
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud)
		VALUES ('hash1', 'blob1', false),
		       ('hash2', 'blob2', true),
			   ('hash3', 'blob3', false);
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	engine := NewAutoDreamSyncEngine(provider)

	// Run process tick
	engine.ProcessForecastTick()

	// Verify sync status
	rows, err := provider.Query(ctx, "SELECT content_hash, synced_to_cloud FROM embedding_cache")
	if err != nil {
		t.Fatalf("Failed to query DB: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var hash string
		var synced bool
		if err := rows.Scan(&hash, &synced); err != nil {
			t.Fatalf("Failed to scan row: %v", err)
		}
		if !synced {
			t.Errorf("Expected hash %s to be synced, but it was false", hash)
		}
	}
}
