package db

import (
	"context"
	"testing"
)

func TestSwarmTasksAndStateMachineMigrations(t *testing.T) {
	// Use an in-memory SQLite database to test degradation logic
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	db, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer db.Close()

	err = db.RunMigrations(context.Background())
	if err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	// Verify swarm_tasks schema degraded correctly
	var count int
	err = db.Provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM swarm_tasks").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query swarm_tasks: %v", err)
	}

	// Verify state_machine_transitions schema degraded correctly
	err = db.Provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM state_machine_transitions").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query state_machine_transitions: %v", err)
	}
}
