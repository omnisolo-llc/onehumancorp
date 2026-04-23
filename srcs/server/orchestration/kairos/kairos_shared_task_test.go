package kairos

import (
    "context"
    "testing"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestKairosSharedTaskRepo(t *testing.T) {
    ctx := context.Background()
    provider := db.NewTestProvider(t)

    // Create the table just like the other tests do, in case migrations drop it.
    _, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    repo := NewSharedTaskRepo(provider)
    task := &SharedTask{
        ID: "test-uuid",
        AgentID: "agent-1",
        Status: "PENDING",
        Payload: []byte(`{"hello":"world"}`),
        CreatedAt: time.Now().Truncate(time.Second).UTC(),
    }

    if err := repo.Insert(ctx, task); err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    fetched, err := repo.Get(ctx, task.ID)
    if err != nil {
        t.Fatalf("failed to get: %v", err)
    }

    if fetched.ID != task.ID || fetched.AgentID != task.AgentID || fetched.Status != task.Status {
        t.Errorf("mismatch: %+v != %+v", fetched, task)
    }
    if string(fetched.Payload) != string(task.Payload) {
        t.Errorf("payload mismatch: %s != %s", string(fetched.Payload), string(task.Payload))
    }
    if !fetched.CreatedAt.Equal(task.CreatedAt) {
        t.Errorf("created_at mismatch: %v != %v", fetched.CreatedAt, task.CreatedAt)
    }
}

func TestSharedTaskRepo_UpdateStatus_GetPendingReview(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			agent_id TEXT,
			status TEXT,
			payload TEXT,
			created_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewSharedTaskRepo(provider)

	// Clean up existing tasks to prevent interference
	provider.Exec(ctx, "DELETE FROM shared_tasks")

	// Insert task with Pending status
	task1 := &SharedTask{
		ID:        "task-1",
		AgentID:   "agent-1",
		Status:    "Pending",
		Payload:   []byte(`{"key":"val1"}`),
		CreatedAt: time.Now().Truncate(time.Second).UTC(),
	}
	if err := repo.Insert(ctx, task1); err != nil {
		t.Fatalf("failed to insert task1: %v", err)
	}

	// Insert task with non-Pending status
	task2 := &SharedTask{
		ID:        "task-2",
		AgentID:   "agent-2",
		Status:    "Approved",
		Payload:   []byte(`{"key":"val2"}`),
		CreatedAt: time.Now().Truncate(time.Second).UTC(),
	}
	if err := repo.Insert(ctx, task2); err != nil {
		t.Fatalf("failed to insert task2: %v", err)
	}

	// Test GetPendingReview
	pendingTasks, err := repo.GetPendingReview(ctx)
	if err != nil {
		t.Fatalf("GetPendingReview failed: %v", err)
	}

	if len(pendingTasks) != 1 {
		t.Fatalf("expected 1 pending task, got %d", len(pendingTasks))
	}
	if pendingTasks[0].ID != "task-1" {
		t.Errorf("expected task-1, got %s", pendingTasks[0].ID)
	}

	// Test UpdateStatus
	if err := repo.UpdateStatus(ctx, "task-1", "Approved"); err != nil {
		t.Fatalf("UpdateStatus failed: %v", err)
	}

	pendingTasksAfterUpdate, err := repo.GetPendingReview(ctx)
	if err != nil {
		t.Fatalf("GetPendingReview failed after update: %v", err)
	}
	if len(pendingTasksAfterUpdate) != 0 {
		t.Fatalf("expected 0 pending tasks after update, got %d", len(pendingTasksAfterUpdate))
	}
}
