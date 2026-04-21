package autodream

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDBTraceQueue(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	// Setup schemas
	_, err = provider.Exec(ctx, `
		CREATE TABLE knowledge_embeddings (id TEXT PRIMARY KEY, content TEXT, metadata TEXT, embedding TEXT);
		CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT);
		CREATE TABLE swarm_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT);
	`)
	if err != nil {
		t.Fatalf("failed to setup mock tables: %v", err)
	}

	queue := NewDBTraceQueue(provider)

	// Fetch when empty
	id, _, _, err := queue.FetchNextTrace(ctx)
	if err == nil || id != "" {
		t.Fatalf("expected error on empty queue")
	}

	// Insert shared task
	_, err = provider.Exec(ctx, `INSERT INTO shared_tasks (id, title, payload, status) VALUES ('task-1', 'title 1', 'payload 1', 'DONE')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Fetch should return task-1
	id, _, meta, err := queue.FetchNextTrace(ctx)
	if err != nil {
		t.Fatalf("failed to fetch task: %v", err)
	}
	if id != "task-1" {
		t.Errorf("expected task-1, got %s", id)
	}
	if meta["title"] != "title 1" {
		t.Errorf("expected title in meta")
	}

	// Mark trace as extracted implicitly by putting it in knowledge_embeddings
	_, err = provider.Exec(ctx, `INSERT INTO knowledge_embeddings (id, content, metadata, embedding) VALUES ('task-1', 'content', '{}', '[]')`)
	if err != nil {
		t.Fatalf("failed to insert knowledge: %v", err)
	}

	// Try fetching again, shouldn't get task-1
	id, _, _, err = queue.FetchNextTrace(ctx)
	if err == nil || id == "task-1" {
		t.Fatalf("task-1 should not be fetched twice")
	}

	// Insert swarm task
	_, err = provider.Exec(ctx, `INSERT INTO swarm_tasks (id, title, payload, status) VALUES ('swarm-1', 'title 2', 'payload 2', 'COMPLETED')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Fetch should return swarm-1
	id, _, meta, err = queue.FetchNextTrace(ctx)
	if err != nil {
		t.Fatalf("failed to fetch task: %v", err)
	}
	if id != "swarm-1" {
		t.Errorf("expected swarm-1, got %s", id)
	}
}
