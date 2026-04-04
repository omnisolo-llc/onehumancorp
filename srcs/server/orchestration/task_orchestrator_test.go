package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func TestTaskOrchestrator(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := NewTestProvider(t)
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

	// Create Task A
	taskA := &models.Task{
		MissionID:   "m1",
		Title:       "Task A",
		Description: "Base task",
		Priority:    "1",
	}
	createdA, err := orchestrator.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if createdA.Status != "READY" {
		t.Fatalf("expected READY, got %s", createdA.Status)
	}

	// Create Task B depending on A
	taskB := &models.Task{
		MissionID:   "m1",
		Title:       "Task B",
		Description: "Dependent task",
		Priority:    "1",
	}
	createdB, err := orchestrator.EnqueueTask(ctx, taskB, []string{createdA.ID})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if createdB.Status != "PENDING" {
		t.Fatalf("expected PENDING, got %s", createdB.Status)
	}

	// Acquire Task A
	acquiredA, err := orchestrator.AcquireReadyTask(ctx, "agent1", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if acquiredA == nil || acquiredA.ID != createdA.ID {
		t.Fatalf("expected Task A to be acquired")
	}
	if acquiredA.Status != "IN_PROGRESS" {
		t.Fatalf("expected IN_PROGRESS, got %s", acquiredA.Status)
	}

	// Try acquiring Task B (should fail, still PENDING)
	acquiredB, err := orchestrator.AcquireReadyTask(ctx, "agent2", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if acquiredB != nil {
		t.Fatalf("expected no ready tasks, got %v", acquiredB.ID)
	}

	// Complete Task A
	err = orchestrator.CompleteTask(ctx, acquiredA.ID, "agent1", "Done A")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Wait for background routine
	time.Sleep(100 * time.Millisecond)

	// Acquire Task B (should now be READY and acquired)
	acquiredB2, err := orchestrator.AcquireReadyTask(ctx, "agent2", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if acquiredB2 == nil || acquiredB2.ID != createdB.ID {
		t.Fatalf("expected Task B to be acquired")
	}

	// Check DB for AutoDream embedding
	var topic string
	err = prov.QueryRow(ctx, "SELECT topic FROM swarm_long_term_memory WHERE topic = $1", "Task Completion: "+acquiredA.ID).Scan(&topic)
	if err != nil {
		t.Fatalf("expected memory to be inserted, got err %v", err)
	}
}

func TestTaskOrchestrator_DistributedLocking(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := NewTestProvider(t)
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

	orchestrator := NewTaskOrchestrator(prov, nil, nil, nil)

	// Create Task
	task := &models.Task{
		MissionID:   "m2",
		Title:       "Task Distributed",
		Description: "Testing Locks",
		Priority:    "1",
	}
	_, err := orchestrator.EnqueueTask(ctx, task, nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Try acquiring concurrently
	agent1Chan := make(chan *models.Task)
	agent2Chan := make(chan *models.Task)

	go func() {
		acq, _ := orchestrator.AcquireReadyTask(ctx, "agent1", nil)
		agent1Chan <- acq
	}()

	go func() {
		acq, _ := orchestrator.AcquireReadyTask(ctx, "agent2", nil)
		agent2Chan <- acq
	}()

	acq1 := <-agent1Chan
	acq2 := <-agent2Chan

	if acq1 != nil && acq2 != nil {
		t.Fatalf("both agents acquired the task, distributed lock failed")
	}

	if acq1 == nil && acq2 == nil {
		t.Fatalf("neither agent acquired the task")
	}
}
