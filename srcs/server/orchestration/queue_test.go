package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskQueue_EnqueueDequeue(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	// Create table for sqlite
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewTaskQueue(prov)

	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   "parent-1",
		Payload:        map[string]interface{}{"data": "value"},
	}

	// Enqueue
	err = q.Enqueue(ctx, task)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task.ID == "" {
		t.Fatal("expected non-empty id")
	}

	// Dequeue
	dequeuedTask, err := q.Dequeue(ctx, "worker-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if dequeuedTask == nil {
		t.Fatal("expected task")
	}
	if dequeuedTask.ID != task.ID {
		t.Errorf("expected id %s, got %s", task.ID, dequeuedTask.ID)
	}
	if dequeuedTask.Payload["data"] != "value" {
		t.Errorf("expected value, got %v", dequeuedTask.Payload["data"])
	}
	if dequeuedTask.WorkerID != "worker-1" {
		t.Errorf("expected worker-1, got %v", dequeuedTask.WorkerID)
	}
	if dequeuedTask.Status != "RUNNING" {
		t.Errorf("expected RUNNING, got %v", dequeuedTask.Status)
	}

	// Dequeue empty
	emptyTask, err := q.Dequeue(ctx, "worker-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if emptyTask != nil {
		t.Fatal("expected nil task")
	}
}

func TestTaskQueue_Acknowledge(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	// Create table for sqlite
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewTaskQueue(prov)

	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   "parent-1",
		Payload:        map[string]interface{}{"data": "value"},
	}

	// Enqueue
	err = q.Enqueue(ctx, task)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Dequeue
	dequeuedTask, err := q.Dequeue(ctx, "worker-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Acknowledge
	err = q.Acknowledge(ctx, dequeuedTask.ID)
	if err != nil {
		t.Fatalf("expected no error on complete, got %v", err)
	}

	// Verify status in DB
	var status string
	err = prov.QueryRow(ctx, "SELECT status FROM sub_agent_queue WHERE id = $1", dequeuedTask.ID).Scan(&status)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if status != "COMPLETED" {
		t.Fatalf("expected COMPLETED, got %v", status)
	}
}
