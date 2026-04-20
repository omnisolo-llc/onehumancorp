package services

import (
	"database/sql"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/apps/api/db"
	_ "modernc.org/sqlite"
)

func retryOnBusy(t *testing.T, fn func() error) {
	for i := 0; i < 10; i++ {
		err := fn()
		if err == nil {
			return
		}
		if err.Error() == "database is locked" {
			time.Sleep(50 * time.Millisecond)
			continue
		}
		t.Fatalf("unexpected error: %v", err)
	}
	t.Fatalf("database remained locked")
}

func setupTestDB(t *testing.T) *sql.DB {
	uri := fmt.Sprintf("file:%s?mode=memory&cache=shared", t.Name())
	conn, err := sql.Open("sqlite", uri)
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	if err := db.CreateSchema(conn, true); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return conn
}

func TestTaskQueueService(t *testing.T) {
	conn := setupTestDB(t)
	defer conn.Close()

	svc := NewTaskQueueService(conn, true)

	t.Run("PushTask", func(t *testing.T) {
		retryOnBusy(t, func() error {
			return svc.PushTask("task-1", "Test Task 1", nil)
		})
	})

	t.Run("ClaimTask", func(t *testing.T) {
		var task *Task
		retryOnBusy(t, func() error {
			var err error
			task, err = svc.ClaimTask("agent-1")
			return err
		})

		if task == nil {
			t.Fatal("expected to claim task, got nil")
		}
		if task.ID != "task-1" {
			t.Errorf("expected task-1, got %s", task.ID)
		}
		if task.Status != "IN_PROGRESS" {
			t.Errorf("expected IN_PROGRESS, got %s", task.Status)
		}
		if *task.AssignedAgent != "agent-1" {
			t.Errorf("expected agent-1, got %s", *task.AssignedAgent)
		}
	})

	t.Run("CompleteTask", func(t *testing.T) {
		retryOnBusy(t, func() error {
			return svc.CompleteTask("task-1")
		})

		var status string
		retryOnBusy(t, func() error {
			return conn.QueryRow("SELECT status FROM shared_tasks WHERE id = 'task-1'").Scan(&status)
		})
		if status != "COMPLETED" {
			t.Errorf("expected COMPLETED, got %s", status)
		}
	})
}
