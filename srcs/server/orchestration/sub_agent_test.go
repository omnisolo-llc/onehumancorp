package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func TestSubAgentSpawner(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure tables exist
	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload TEXT NOT NULL DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	spawner := NewSubAgentSpawner(prov, nil, nil)

	task := &models.Task{
		ID:        "sub-task-1",
		MissionID: "m1",
		Title:     "Sub Agent Work",
		Status:    "IN_PROGRESS",
	}

	_, _ = prov.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ($1, $2, $3, $4)", task.ID, task.MissionID, task.Title, task.Status)

	err := spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("expected no error from Spawn, got: %v", err)
	}

	// Wait for the background routine to finish
	time.Sleep(200 * time.Millisecond)

	var status string
	err = prov.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", task.ID).Scan(&status)
	if err != nil {
		t.Fatalf("expected to query task status, got err: %v", err)
	}

	if status != "COMPLETED" {
		t.Fatalf("expected task status COMPLETED, got %s", status)
	}
}

func TestSubAgentDelegationInOrchestrator(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure tables exist
	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload TEXT NOT NULL DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
    _, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)

	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
			id TEXT PRIMARY KEY,
			topic TEXT NOT NULL,
			summary TEXT NOT NULL,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	orchestrator := NewTaskOrchestrator(prov, nil, nil, nil)
	defer orchestrator.(*DefaultTaskOrchestrator).Stop()

	// Create Task that is delegated
	task := &models.Task{
		MissionID:   "m2",
		Title:       "Delegated Task",
		Description: "Should be picked up by SubAgentSpawner",
		Priority:    "DELEGATED", // triggers the delegated worker logic
	}
	created, err := orchestrator.EnqueueTask(ctx, task, nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Give the orchestrator's background worker time to pick it up and complete it via the SubAgentSpawner
	time.Sleep(300 * time.Millisecond)

	var status, assigned string
	err = prov.QueryRow(ctx, "SELECT status, assigned_agent_id FROM swarm_tasks WHERE id = $1", created.ID).Scan(&status, &assigned)
	if err != nil {
		t.Fatalf("expected to query task, got err: %v", err)
	}

	if assigned != "SUB_AGENT_WORKER" {
		t.Fatalf("expected assigned_agent_id to be SUB_AGENT_WORKER, got %s", assigned)
	}

	if status != "COMPLETED" {
		t.Fatalf("expected task status to be COMPLETED, got %s", status)
	}
}