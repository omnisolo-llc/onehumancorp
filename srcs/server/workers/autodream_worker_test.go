package workers

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupAutoDreamTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)

	query := `CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		payload TEXT
	);

	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		task_id TEXT,
		organization_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		metadata TEXT,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	);`

	// Create the tables again with another name just in case? Or delete them.
	provider.Exec(context.Background(), "DROP TABLE IF EXISTS shared_tasks_decomposition")
	provider.Exec(context.Background(), "DROP TABLE IF EXISTS autodream_memories")

	_, err = provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	return provider
}

func TestAutoDreamWorker_ConsolidateMemories(t *testing.T) {
	provider := setupAutoDreamTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	provider.Exec(ctx, "DELETE FROM shared_tasks_decomposition")
	provider.Exec(ctx, "DELETE FROM autodream_memories")

	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status, payload) VALUES (?, ?, ?, ?, ?, ?)",
		"task-1", "org-1", "Test Task", "Test Desc", "COMPLETED", "{}")
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	worker.consolidateMemories(ctx)

	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}

	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = 'task-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}

	if status != "CONSOLIDATED" {
		t.Errorf("expected status 'CONSOLIDATED', got %s", status)
	}
}
