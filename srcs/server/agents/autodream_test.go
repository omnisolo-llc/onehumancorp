package agents

import (
	"context"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamEngine(t *testing.T) {
	ctx := context.Background()

	// Setup SQLite DB for testing
	dbPath := filepath.Join(t.TempDir(), "test.db")
	database, err := db.NewSqliteProviderForTest(dbPath)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer database.Close()

	// Apply schemas
	_, err = database.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING'
		);
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			source_mission_id TEXT,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	// Insert dummy task
	_, err = database.Exec(ctx, "INSERT INTO shared_tasks (id, mission_id, title, description, status) VALUES ('task-1', 'mission-1', 'Test', 'Testing', 'COMPLETED')")
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	engine := NewAutoDreamEngine(database, nil)

	// Consolidate
	engine.ConsolidateMemories(ctx)

	// Verify
	row := database.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories")
	var count int
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 memory, got %d", count)
	}
}
