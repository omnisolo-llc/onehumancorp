package mcp

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/sync"
)

func TestMCPSyncer_SyncDeltas_SQLite(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	if err := dbWrapper.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	syncer := NewSyncer(dbWrapper)

	now := time.Now()
	deltas := []sync.SyncDelta{
		{
			ID:        "d1",
			EntityID:  "e1",
			Data:      "{}",
			UpdatedAt: now,
		},
	}

	err = syncer.SyncDeltas(ctx, deltas)
	if err != nil {
		t.Fatalf("failed to sync deltas: %v", err)
	}

	var count int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM crdt_deltas WHERE synced_to_cloud = false").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 delta inserted with synced_to_cloud = false, got %d", count)
	}
}

type mockPGProvider struct {
	db.Provider
}

func (m *mockPGProvider) IsSQLite() bool {
	return false
}

func (m *mockPGProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	// For testing, just return success
	return 1, nil
}

func TestMCPSyncer_SyncDeltas_Postgres(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	ctx := context.Background()

	// Use mock PG provider for testing Postgres logic
	dbWrapper := &db.DB{Provider: &mockPGProvider{}}

	syncer := NewSyncer(dbWrapper)

	now := time.Now()
	deltas := []sync.SyncDelta{
		{
			ID:        "d2",
			EntityID:  "e2",
			Data:      "{}",
			UpdatedAt: now,
		},
	}

	err := syncer.SyncDeltas(ctx, deltas)
	if err != nil {
		t.Fatalf("failed to sync deltas: %v", err)
	}
}
