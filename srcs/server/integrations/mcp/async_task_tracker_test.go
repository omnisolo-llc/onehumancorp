package mcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAsyncTaskTracker(t *testing.T) {
	pool := db.NewTestProvider(t)
	defer pool.Close()
	// Run migrations to create table
	dbWrapper := &db.DB{Provider: pool}
	if err := dbWrapper.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tracker := NewAsyncTaskTracker(pool)
	ctx := context.Background()

	task := AsyncTask{
		ID:       "task-1",
		TenantID: "tenant-1",
		AgentID:  "agent-1",
		Status:   "pending",
		Payload:  `{"foo":"bar"}`,
	}

	err := tracker.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	fetched, err := tracker.GetTask(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get task: %v", err)
	}
	if fetched == nil {
		t.Fatal("task not found")
	}
	if fetched.Status != "pending" {
		t.Errorf("expected pending, got %s", fetched.Status)
	}

	err = tracker.UpdateTaskStatus(ctx, task.ID, "completed", `{"result":"ok"}`)
	if err != nil {
		t.Fatalf("failed to update task: %v", err)
	}

	fetched, err = tracker.GetTask(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get task after update: %v", err)
	}
	if fetched == nil {
		t.Fatal("task not found after update")
	}
	if fetched.Status != "completed" {
		t.Errorf("expected completed, got %s", fetched.Status)
	}
	if fetched.Payload != `{"result":"ok"}` {
		t.Errorf("expected payload {\"result\":\"ok\"}, got %s", fetched.Payload)
	}
}
