package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/ohc-api/srcs/server/db"
	"github.com/onehumancorp/ohc-api/srcs/server/models"
)

func TestSubAgentSpawner(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure tables exist
	_, err := prov.Exec(ctx, `
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
	if err != nil {
		t.Fatalf("Failed to create table swarm_tasks: %v", err)
	}

	_, err = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table task_dependencies: %v", err)
	}

	// Initialize orchestrator with nil dependencies for Redis/Hub/Mesh
	// The orchestrator creates its own spawner.
	to := NewTaskOrchestrator(prov, nil, nil, nil)
	defaultOrchestrator, ok := to.(*DefaultTaskOrchestrator)
	if !ok {
		t.Fatalf("Expected DefaultTaskOrchestrator")
	}
	defer defaultOrchestrator.Stop()

	// 1. Enqueue a task with DELEGATED priority
	task := &models.Task{
		ID:          "task-delegated-1",
		MissionID:   "mission-1",
		Title:       "Sub-Agent Work",
		Description: "Perform isolated operation",
		Priority:    "DELEGATED",
	}

	enqueued, err := to.EnqueueTask(ctx, task, nil)
	if err != nil {
		t.Fatalf("Failed to enqueue task: %v", err)
	}

	// 2. The background worker should pick it up and run it via the spawner.
	// We check if the task completes eventually.
	timeout := time.After(5 * time.Second)
	ticker := time.NewTicker(200 * time.Millisecond)
	defer ticker.Stop()

	taskCompleted := false
	for {
		select {
		case <-timeout:
			t.Fatalf("Timed out waiting for sub-agent task to complete")
		case <-ticker.C:
			var status string
			err := prov.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", enqueued.ID).Scan(&status)
			if err != nil {
				t.Fatalf("Failed to query task status: %v", err)
			}
			if status == "COMPLETED" {
				taskCompleted = true
			}
		}
		if taskCompleted {
			break
		}
	}

	if !taskCompleted {
		t.Fatalf("Task should be completed by the sub-agent spawner")
	}
}
