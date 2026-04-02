package agents

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupAutoDreamDB(t *testing.T) (db.Provider, func()) {
	t.Helper()
	os.Setenv("DATABASE_URL", "sqlite://file:test_autodream_db?mode=memory&cache=shared")
	testDB, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create memory db: %v", err)
	}
	var prov db.Provider = testDB.Provider

	_, err = prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	return prov, func() {
		prov.Close()
	}
}

func TestAutoDreamEngine_Consolidate(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov, cleanup := setupAutoDreamDB(t)
	defer cleanup()

	ctx := context.Background()

	// Insert completed task
	_, err := prov.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, title, description, status)
		VALUES ('task-1', 'mission-1', 'Test Task', 'Desc', 'COMPLETED')
	`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	engine := NewAutoDreamEngine(prov, nil)
	err = engine.consolidate(ctx)
	if err != nil {
		t.Fatalf("consolidate failed: %v", err)
	}

	// Verify memory created
	var count int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'mission-1'").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 memory, got %d, err: %v", count, err)
	}

	// Consolidate again, should skip
	err = engine.consolidate(ctx)
	if err != nil {
		t.Fatalf("consolidate failed: %v", err)
	}

	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'mission-1'").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 memory after second run, got %d", count)
	}
}
