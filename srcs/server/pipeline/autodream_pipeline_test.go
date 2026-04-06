package pipeline

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamPipeline_Run(t *testing.T) {
	pool, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	defer pool.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Initialize tables needed by AutoDreamPipeline
	_, err = pool.Exec(ctx, `
		CREATE TABLE agent_session_data (
			session_id TEXT PRIMARY KEY,
			context_data TEXT,
			last_accessed DATETIME
		);
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE swarm_dream_epochs (
			id TEXT PRIMARY KEY,
			status TEXT,
			cluster_results TEXT,
			created_at DATETIME,
			completed_at DATETIME
		);
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			updated_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	// Insert some mock data
	_, err = pool.Exec(ctx, `
		INSERT INTO agent_session_data (session_id, context_data, last_accessed)
		VALUES ('sess1', 'User discussed feature X', CURRENT_TIMESTAMP)
	`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	pipeline := NewAutoDreamPipeline(pool)
	err = pipeline.Run(ctx)
	if err != nil {
		t.Fatalf("Run() failed: %v", err)
	}

	// Verify that consolidated memory was created
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count consolidated_memory: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 consolidated memory, got %d", count)
	}
}
