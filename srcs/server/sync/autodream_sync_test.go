package sync

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

func TestAutoDreamSync_ProcessForecastTick(t *testing.T) {
	// Initialize telemetry to avoid panics on nil counters
	meterProvider := otel.GetMeterProvider()
	meter := meterProvider.Meter("test")
	err := telemetry.InitWithMeter(meter)
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}

	// Set up SQLite memory DB
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	// Ensure table is empty
	_, _ = pool.Exec(ctx, "DELETE FROM embedding_cache")

	// Insert an unsynced record
	_, err = pool.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES (?, ?, ?)", "hash1", "[0.1, 0.2]", false)
	if err != nil {
		t.Fatalf("failed to insert record: %v", err)
	}

	// Insert a synced record
	_, err = pool.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES (?, ?, ?)", "hash2", "[0.3, 0.4]", true)
	if err != nil {
		t.Fatalf("failed to insert record: %v", err)
	}

	// Verify insertion
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 unsynced record before sync, got %d (err: %v)", count, err)
	}

	// Run sync
	syncEngine := NewAutoDreamSync(pool.Provider)
	syncEngine.ProcessForecastTick(ctx)

	// Verify sync success
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&count)
	if err != nil || count != 0 {
		t.Errorf("expected 0 unsynced records after sync, got %d (err: %v)", count, err)
	}

	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = true").Scan(&count)
	if err != nil || count != 2 {
		t.Errorf("expected 2 synced records after sync, got %d (err: %v)", count, err)
	}
}
