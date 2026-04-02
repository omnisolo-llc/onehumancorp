package orchestration

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// newTestSqliteProvider is a helper for testing
func newTestSqliteProvider() (db.Provider, error) {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		return nil, err
	}
	sqlDB.SetMaxOpenConns(1)
	return db.NewSqliteProvider(sqlDB), nil
}

func TestClaimTask(t *testing.T) {
	// Use an in-memory SQLite database for testing SIPDB
	provider, err := newTestSqliteProvider()
	if err != nil {
		t.Skip("skipping test: could not create memory provider")
	}

	// Create test tables
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL,
			assigned_agent_id TEXT,
			locked_until DATETIME,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_tasks: %v", err)
	}

	sipDB, _ := NewSIPDBWithProvider(provider)
	hub := NewHub()
	hub.SetSIPDB(sipDB)

	// Insert a dummy task
	_, err = provider.Exec(context.Background(), `
		INSERT INTO swarm_tasks (id, mission_id, title, status, assigned_agent_id, payload)
		VALUES ('task1', 'mission1', 'Test Task', 'PENDING', NULL, '{}')
	`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	ctx := context.Background()

	// Agent 1 claims task
	claimed, err := hub.ClaimTask(ctx, "task1", "agent1")
	if err != nil {
		t.Fatalf("ClaimTask returned error: %v", err)
	}
	if !claimed {
		t.Errorf("Expected agent1 to claim task, got false")
	}

	// Agent 2 tries to claim same task
	claimed2, err := hub.ClaimTask(ctx, "task1", "agent2")
	if err != nil {
		t.Fatalf("ClaimTask returned error: %v", err)
	}
	if claimed2 {
		t.Errorf("Expected agent2 to fail claiming already-claimed task, got true")
	}
}
