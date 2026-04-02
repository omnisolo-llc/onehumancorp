package orchestration

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSwarmTaskStore(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	store := NewSwarmTaskStore(pool.Provider, nil, nil)
	ctx := context.Background()

	missionID := "mission-test-task"

	_, _ = pool.Provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, 'PENDING', '{}')", missionID)

	task := &SwarmTask{
		MissionID:    missionID,
		Title:        "Test Task",
		Status:       TaskStatusPending,
		Dependencies: []string{},
		Payload:      json.RawMessage("{}"),
	}

	err = store.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	// Fetch task ID since UUID is auto-generated
	var taskID string
	err = pool.Provider.QueryRow(ctx, "SELECT id FROM swarm_tasks WHERE title = 'Test Task'").Scan(&taskID)
	if err != nil {
		t.Fatalf("failed to get task ID: %v", err)
	}

	// Claim
	claimed, err := store.ClaimTask(ctx, taskID, "agent-1")
	if err != nil {
		t.Fatalf("claim failed: %v", err)
	}
	if !claimed {
		t.Fatal("expected to claim task")
	}

	// Attempt claim again
	claimed2, err := store.ClaimTask(ctx, taskID, "agent-2")
	if err != nil {
		t.Fatalf("claim failed: %v", err)
	}
	if claimed2 {
		t.Fatal("expected second claim to fail")
	}

	// Complete
	err = store.CompleteTask(ctx, taskID)
	if err != nil {
		t.Fatalf("failed to complete task: %v", err)
	}

	var status string

	err = pool.Provider.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", taskID).Scan(&status)

	if err != nil || status != "COMPLETED" {
		t.Fatalf("expected COMPLETED status, got %s (err: %v)", status, err)
	}
}
