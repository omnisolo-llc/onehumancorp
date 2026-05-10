package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/google/uuid"
	_ "github.com/mattn/go-sqlite3"
)

func setupQueueTestDB(t *testing.T) *sql.DB {
	dbPath := filepath.Join(t.TempDir(), "test_queue.db")
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	migrationPath := "../db/migrations/058_kairos_phase4_sub_agent_queue_sqlite.sql"
	migrationData, err := os.ReadFile(migrationPath)
	if err != nil {
		// Try fallback if running directly via go test in orchestration folder
		migrationPath = "../db/migrations/058_kairos_phase4_sub_agent_queue_sqlite.sql"
		if _, err2 := os.Stat(migrationPath); os.IsNotExist(err2) {
			t.Fatalf("migration file not found: %v", err)
		}
	}

	// Poor man's goose up for sqlite
	query := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		worker_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`

	_ = migrationData // In real execution we'd parse goose format, but for sqlite unit tests, manual string is robust

	if _, err := db.Exec(query); err != nil {
		t.Fatalf("failed to execute migration: %v", err)
	}

	return db
}

func TestDBTaskQueue_Enqueue(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewDBTaskQueue(db, false, "worker-1")
	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   uuid.New().String(),
		Payload:        json.RawMessage(`{"cmd": "echo test"}`),
	}

	err := queue.Enqueue(ctx, task)
	if err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	if task.ID == "" {
		t.Errorf("expected ID to be set")
	}

	if task.Status != "QUEUED" {
		t.Errorf("expected status to be QUEUED")
	}
}

func TestDBTaskQueue_Dequeue(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewDBTaskQueue(db, false, "worker-1")
	ctx := context.Background()

	// Try dequeueing from empty queue
	task, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue empty queue failed: %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task from empty queue")
	}

	// Enqueue a task
	enqueuedTask := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   uuid.New().String(),
	}
	if err := queue.Enqueue(ctx, enqueuedTask); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	// Dequeue the task
	task, err = queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if task == nil {
		t.Fatalf("expected to dequeue task, got nil")
	}

	if task.ID != enqueuedTask.ID {
		t.Errorf("expected to dequeue task %s, got %s", enqueuedTask.ID, task.ID)
	}
	if task.Status != "RUNNING" {
		t.Errorf("expected status to be RUNNING, got %s", task.Status)
	}
	if task.WorkerID != "worker-1" {
		t.Errorf("expected worker ID to be worker-1, got %s", task.WorkerID)
	}

	// Dequeue again, should be empty
	task2, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue second time failed: %v", err)
	}
	if task2 != nil {
		t.Errorf("expected nil task, got %v", task2)
	}
}

func TestDBTaskQueue_Acknowledge(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewDBTaskQueue(db, false, "worker-1")
	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   uuid.New().String(),
	}
	if err := queue.Enqueue(ctx, task); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	dequeuedTask, err := queue.Dequeue(ctx)
	if err != nil || dequeuedTask == nil {
		t.Fatalf("Dequeue failed: %v", err)
	}

	err = queue.Acknowledge(ctx, dequeuedTask.ID)
	if err != nil {
		t.Fatalf("Acknowledge failed: %v", err)
	}

	// Verify status in DB
	var status string
	err = db.QueryRow("SELECT status FROM sub_agent_queue WHERE id = ?", dequeuedTask.ID).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "COMPLETED" {
		t.Errorf("expected status COMPLETED, got %s", status)
	}
}

func TestDBTaskQueue_FailTask(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewDBTaskQueue(db, false, "worker-1")
	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		ParentTaskID:   uuid.New().String(),
	}
	if err := queue.Enqueue(ctx, task); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	dequeuedTask, err := queue.Dequeue(ctx)
	if err != nil || dequeuedTask == nil {
		t.Fatalf("Dequeue failed: %v", err)
	}

	err = queue.FailTask(ctx, dequeuedTask.ID)
	if err != nil {
		t.Fatalf("FailTask failed: %v", err)
	}

	// Verify status in DB
	var status string
	err = db.QueryRow("SELECT status FROM sub_agent_queue WHERE id = ?", dequeuedTask.ID).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "FAILED" {
		t.Errorf("expected status FAILED, got %s", status)
	}
}

func TestDBTaskQueue_ContextCancellation(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewDBTaskQueue(db, false, "worker-1")

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	task := &Task{OrganizationID: "org-1", ParentTaskID: uuid.New().String()}
	err := queue.Enqueue(ctx, task)
	if err == nil {
		t.Errorf("expected error due to canceled context")
	}

	_, err = queue.Dequeue(ctx)
	if err == nil {
		t.Errorf("expected error due to canceled context")
	}
}
