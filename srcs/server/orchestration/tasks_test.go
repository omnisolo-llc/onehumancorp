package orchestration


import (
	"context"
	"database/sql"
	"os"
	"testing"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestProvider(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}
	return db.NewSqliteProvider(sqliteDB)
}

func setupTestDB(t *testing.T) (*TaskManager, func()) {
	t.Helper()
	// Create an in-memory SQLite database
	prov := setupTestProvider(t)

	// Create tables
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			payload TEXT NOT NULL DEFAULT '{}',
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	tm := NewTaskManager(prov, nil)

	return tm, func() {
		prov.Close()
	}
}

func TestTaskManager_CreateTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()
	task, err := tm.CreateTask(ctx, "mission-1", "Test Task", "Desc", "P1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.Title != "Test Task" {
		t.Errorf("expected Title 'Test Task', got %s", task.Title)
	}
	if task.Status != "PENDING" {
		t.Errorf("expected Status 'PENDING', got %s", task.Status)
	}
}

func TestTaskManager_ClaimTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()

	// Claim when empty
	task, err := tm.ClaimTask(ctx, "non-existent-task-id", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task when empty, got %v", task)
	}

	// Create task
	createdTask, err := tm.CreateTask(ctx, "mission-1", "Test Task", "Desc", "P1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Claim task
	claimedTask, err := tm.ClaimTask(ctx, createdTask.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if claimedTask.Status != "IN_PROGRESS" {
		t.Errorf("expected Status 'IN_PROGRESS', got %s", claimedTask.Status)
	}
	if claimedTask.AssignedAgentID != "agent-1" {
		t.Errorf("expected AssignedAgentID 'agent-1', got %s", claimedTask.AssignedAgentID)
	}

	// Claim another (should be empty)
	task3, err := tm.ClaimTask(ctx, "another-non-existent-id", "agent-2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task3 != nil {
		t.Fatalf("expected nil task, got %v", task3)
	}
}

func TestTaskManager_CompleteTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()
	task, _ := tm.CreateTask(ctx, "mission-1", "Test Task", "Desc", "P1")
	claimedTask, _ := tm.ClaimTask(ctx, task.ID, "agent-1")

	if claimedTask.ID != task.ID {
		t.Fatalf("claimed task id mismatch")
	}

	// Complete task
	err := tm.CompleteTask(ctx, claimedTask.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Try completing again
	err = tm.CompleteTask(ctx, claimedTask.ID, "agent-1")
	if err == nil {
		t.Fatalf("expected error when completing an already completed task")
	}

	// Complete non-existent task
	err = tm.CompleteTask(ctx, "non-existent", "agent-1")
	if err == nil {
		t.Fatalf("expected error when completing non-existent task")
	}
}
