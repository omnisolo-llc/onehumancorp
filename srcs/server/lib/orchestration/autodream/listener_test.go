package autodream

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestListener(t *testing.T) {
	ctx := context.Background()
	provider := NewTestProvider(t)

	schema := `
		CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert a DONE task
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks_v2 (id, organization_id, title, description, status)
		VALUES ('task-1', 'org-1', 'Test Done', 'Desc', 'DONE')
	`)
	if err != nil {
		t.Fatalf("Failed to insert mock task: %v", err)
	}

	listener := NewListener(provider)

	err = listener.BatchCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Test the background start (just don't block)
	ctxCancel, cancel := context.WithCancel(ctx)
	listener.StartBackgroundListener(ctxCancel, 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()
}
