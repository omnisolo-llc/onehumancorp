package orchestration

import (
	"context"
	"os"
	"testing"
)

func TestTaskManager_CreateSharedTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()

	// Need to create shared_tasks table for tests
	_, err := tm.db.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	task, err := tm.CreateSharedTask(ctx, "org-1", "Test Title", "Test Desc", "P1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.Title != "Test Title" {
		t.Errorf("expected Title 'Test Title', got %s", task.Title)
	}
	if task.Status != "PENDING" {
		t.Errorf("expected Status 'PENDING', got %s", task.Status)
	}
}

func TestTaskManager_ClaimSharedTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()

	_, err := tm.db.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	task, err := tm.ClaimSharedTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task when empty, got %v", task)
	}

	createdTask, err := tm.CreateSharedTask(ctx, "org-1", "Test Title", "Test Desc", "P1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	claimedTask, err := tm.ClaimSharedTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if claimedTask.ID != createdTask.ID {
		t.Errorf("expected ID %s, got %s", createdTask.ID, claimedTask.ID)
	}
	if claimedTask.Status != "IN_PROGRESS" {
		t.Errorf("expected Status 'IN_PROGRESS', got %s", claimedTask.Status)
	}
	if claimedTask.AgentID != "agent-1" {
		t.Errorf("expected AgentID 'agent-1', got %s", claimedTask.AgentID)
	}
}

func TestTaskManager_CompleteSharedTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()

	_, err := tm.db.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	task, _ := tm.CreateSharedTask(ctx, "org-1", "Test Title", "Test Desc", "P1")
	claimedTask, _ := tm.ClaimSharedTask(ctx, "org-1", "agent-1")

	if claimedTask.ID != task.ID {
		t.Fatalf("claimed task id mismatch")
	}

	err = tm.CompleteSharedTask(ctx, claimedTask.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = tm.CompleteSharedTask(ctx, claimedTask.ID, "agent-1")
	if err == nil {
		t.Fatalf("expected error when completing an already completed task")
	}
}
