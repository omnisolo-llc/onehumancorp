package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTasksTestDB(t *testing.T) (*TaskManager, func()) {
	t.Helper()
	// Create an in-memory SQLite database
	prov := db.NewTestProvider(t)

	// Create tables
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT NOT NULL DEFAULT '{}',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
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

	tm, cleanup := setupTasksTestDB(t)
	defer cleanup()

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})
	task, err := tm.CreateTask(ctx, "org-1", "Test Task", "Desc", "P1")
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

	tm, cleanup := setupTasksTestDB(t)
	defer cleanup()

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})

	// Claim when empty
	task, err := tm.ClaimTask(ctx, "non-existent-task-id", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task when empty, got %v", task)
	}

	// Create task
	createdTask, err := tm.CreateTask(ctx, "org-1", "Test Task", "Desc", "P1")
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

func TestTaskManager_PollTasks(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTasksTestDB(t)
	defer cleanup()

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})

	// Poll when empty
	tasks, err := tm.PollTasks(ctx, "agent-1", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks) != 0 {
		t.Fatalf("expected empty tasks, got %d", len(tasks))
	}

	// Create a few tasks with different priorities
	_, _ = tm.CreateTask(ctx, "org-1", "Task 1", "Desc", "P2")
	_, _ = tm.CreateTask(ctx, "org-1", "Task 2", "Desc", "P1") // Should be polled first
	_, _ = tm.CreateTask(ctx, "org-1", "Task 3", "Desc", "P3")

	// Poll tasks with limit 2
	tasks, err = tm.PollTasks(ctx, "agent-1", 2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks) != 2 {
		t.Fatalf("expected 2 tasks, got %d", len(tasks))
	}

	// Verify priority ordering
	if tasks[0].Priority != "P1" {
		t.Errorf("expected P1 task first, got %s", tasks[0].Priority)
	}
	if tasks[1].Priority != "P2" {
		t.Errorf("expected P2 task second, got %s", tasks[1].Priority)
	}

	// All should be marked IN_PROGRESS assigned to agent-1
	for _, task := range tasks {
		if task.Status != "IN_PROGRESS" {
			t.Errorf("expected task status IN_PROGRESS, got %s", task.Status)
		}
		if task.AssignedAgentID != "agent-1" {
			t.Errorf("expected assigned agent agent-1, got %s", task.AssignedAgentID)
		}
	}

	// Poll remaining tasks
	tasks2, err := tm.PollTasks(ctx, "agent-2", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks2) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks2))
	}
	if tasks2[0].Priority != "P3" {
		t.Errorf("expected P3 task, got %s", tasks2[0].Priority)
	}
	if tasks2[0].AssignedAgentID != "agent-2" {
		t.Errorf("expected assigned agent agent-2, got %s", tasks2[0].AssignedAgentID)
	}
}

func TestTaskManager_PollTasks_Dependencies(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})

	// Create a parent task and a dependent task
	parentTask, _ := tm.CreateTask(ctx, "org-1", "Parent Task", "Desc", "P1")
	dependentTask, _ := tm.CreateTask(ctx, "org-1", "Dependent Task", "Desc", "P1")

	// Add dependency
	_, err := tm.db.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", dependentTask.ID, parentTask.ID)
	if err != nil {
		t.Fatalf("failed to insert dependency: %v", err)
	}

	// Poll should only return the parent task because dependent is blocked
	tasks, err := tm.PollTasks(ctx, "agent-1", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks))
	}
	if tasks[0].ID != parentTask.ID {
		t.Fatalf("expected to poll parent task, got %s", tasks[0].ID)
	}

	// Complete the parent task
	err = tm.CompleteTask(ctx, parentTask.ID, "agent-1")
	if err != nil {
		t.Fatalf("failed to complete parent task: %v", err)
	}

	// Now poll should return the dependent task
	tasks2, err := tm.PollTasks(ctx, "agent-1", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks2) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks2))
	}
	if tasks2[0].ID != dependentTask.ID {
		t.Fatalf("expected to poll dependent task, got %s", tasks2[0].ID)
	}
}

func TestTaskManager_CompleteTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTasksTestDB(t)
	defer cleanup()

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})
	task, _ := tm.CreateTask(ctx, "org-1", "Test Task", "Desc", "P1")
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
