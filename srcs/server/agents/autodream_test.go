package agents

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamEngine_ProcessAutoDreamTick(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	os.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")

	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer provider.Close()

	if err := provider.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Insert a completed task
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, created_at, updated_at)
		VALUES ('task-1', 'miss-1', 'Test', 'Desc', 'COMPLETED', 'P1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`)
	if err != nil {
		t.Fatalf("failed to insert mock task: %v", err)
	}

	engine := NewAutoDreamEngine(provider, "test-api-key")
	engine.ProcessAutoDreamTick(ctx)

	// Check if memory was consolidated
	row := provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_mission_id = 'task-1'")
	var count int
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory consolidated, got %d", count)
	}
}
