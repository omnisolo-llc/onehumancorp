package orchestration

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestProcessEvent_SubTaskCompleted(t *testing.T) {
	ctx := context.Background()
	provider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			parent_task_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING'
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	parentID := "parent-1"
	sub1ID := "sub-1"
	sub2ID := "sub-2"

	// Insert parent and two subtasks
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ($1, 'org-1', 'Parent Task', 'EXECUTING')", parentID)
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, parent_task_id, title, status) VALUES ($1, 'org-1', $2, 'Subtask 1', 'DONE')", sub1ID, parentID)
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, parent_task_id, title, status) VALUES ($1, 'org-1', $2, 'Subtask 2', 'IN_PROGRESS')", sub2ID, parentID)

	sm := NewTaskStateMachine(provider)

	// First subtask completes, but subtask 2 is still IN_PROGRESS. Parent should remain EXECUTING.
	err = sm.ProcessEvent(ctx, sub1ID, EventSubTaskCompleted)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var parentStatus string
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", parentID).Scan(&parentStatus)
	if parentStatus != "EXECUTING" {
		t.Fatalf("expected parent status to be EXECUTING, got %s", parentStatus)
	}

	// Subtask 2 completes
	provider.Exec(ctx, "UPDATE shared_tasks SET status = 'DONE' WHERE id = $1", sub2ID)
	err = sm.ProcessEvent(ctx, sub2ID, EventSubTaskCompleted)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Now all subtasks are DONE, parent should transition to VERIFYING
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", parentID).Scan(&parentStatus)
	if parentStatus != "VERIFYING" {
		t.Fatalf("expected parent status to be VERIFYING, got %s", parentStatus)
	}
}

func TestProcessEvent_Concurrent(t *testing.T) {
	ctx := context.Background()
	// Use file-based shared in-memory db so concurrent connections see the same tables
	provider, err := db.NewSQLiteProvider("file:memdb2?mode=memory&cache=shared")
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			parent_task_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING'
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	parentID := "parent-1"
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ($1, 'org-1', 'Parent Task', 'EXECUTING')", parentID)

	numSubTasks := 10
	subTaskIDs := make([]string, numSubTasks)
	for i := 0; i < numSubTasks; i++ {
		id := fmt.Sprintf("sub-%d", i)
		subTaskIDs[i] = id
		provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, parent_task_id, title, status) VALUES ($1, 'org-1', $2, 'Subtask', 'IN_PROGRESS')", id, parentID)
	}

	sm := NewTaskStateMachine(provider)

	var wg sync.WaitGroup
	for _, id := range subTaskIDs {
		wg.Add(1)
		go func(taskID string) {
			defer wg.Done()

			// Simulate subtask completing
			// Execute with retries due to SQLite lock contention
			for retries := 0; retries < 10; retries++ {
				_, err := provider.Exec(ctx, "UPDATE shared_tasks SET status = 'DONE' WHERE id = $1", taskID)
				if err == nil {
					break
				}
				time.Sleep(time.Millisecond * 10)
			}

			for retries := 0; retries < 10; retries++ {
				err := sm.ProcessEvent(ctx, taskID, EventSubTaskCompleted)
				if err == nil || !strings.Contains(err.Error(), "database is locked") {
					break
				}
				time.Sleep(time.Millisecond * 10)
			}
		}(id)
	}

	wg.Wait()

	// Wait a bit to ensure all concurrent transactions are fully committed to SQLite memory DB
	time.Sleep(time.Millisecond * 100)

	var parentStatus string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", parentID).Scan(&parentStatus)
	if err != nil {
		t.Fatalf("failed to query parent status: %v", err)
	}
	if parentStatus != "VERIFYING" {
		t.Fatalf("expected parent status to be VERIFYING after all subtasks complete, got %s", parentStatus)
	}
}
