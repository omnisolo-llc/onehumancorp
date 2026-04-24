package tasks

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskDecompositionService(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	// Need to run migration or create table manually for tests since db.NewTestProvider might not have this schema
	// For testing, we create the table first
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			assigned_agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload JSONB,
			parent_plan_id TEXT,
			dependencies JSONB NOT NULL DEFAULT '[]',
			locked_until TIMESTAMP,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	t.Run("CreateTask", func(t *testing.T) {
		// Clean up the table before each test run
		_, _ = provider.Exec(ctx, "DELETE FROM shared_tasks_decomposition")
		task := &SharedTaskDecomposition{
			ID:             "task-1",
			OrganizationID: "org-1",
			Title:          "Test Task",
			Description:    "Description",
			Status:         TaskStatusPending,
			Priority:       "P1",
			Payload:        []byte(`{"key": "value"}`),
			Dependencies:   []byte(`[]`),
			CreatedAt:      time.Now(),
			UpdatedAt:      time.Now(),
		}

		err := svc.CreateTask(ctx, task)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		retrieved, err := svc.GetTask(ctx, "task-1")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if retrieved.ID != "task-1" {
			t.Errorf("expected ID task-1, got %v", retrieved.ID)
		}
	})

	t.Run("ClaimTask", func(t *testing.T) {
		// Clean up the table before each test run
		_, _ = provider.Exec(ctx, "DELETE FROM shared_tasks_decomposition")
		task := &SharedTaskDecomposition{
			ID:             "task-2",
			OrganizationID: "org-1",
			Title:          "Test Task 2",
			Description:    "Description 2",
			Status:         TaskStatusPending,
			Priority:       "P1",
			Payload:        []byte(`{}`),
			Dependencies:   []byte(`[]`),
			CreatedAt:      time.Now(),
			UpdatedAt:      time.Now(),
		}

		err := svc.CreateTask(ctx, task)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		claimed, err := svc.ClaimTask(ctx, "org-1", "agent-1")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if claimed == nil {
			t.Fatalf("expected claimed task, got nil")
		}

		if claimed.ID != "task-2" {
			t.Errorf("expected ID task-2, got %v", claimed.ID)
		}
		if claimed.Status != TaskStatusClaimed {
			t.Errorf("expected status CLAIMED, got %v", claimed.Status)
		}
		if *claimed.AssignedAgentID != "agent-1" {
			t.Errorf("expected assigned agent agent-1, got %v", *claimed.AssignedAgentID)
		}
	})

	t.Run("UpdateTaskStatus", func(t *testing.T) {
		// Clean up the table before each test run
		_, _ = provider.Exec(ctx, "DELETE FROM shared_tasks_decomposition")
		task := &SharedTaskDecomposition{
			ID:             "task-3",
			OrganizationID: "org-1",
			Title:          "Test Task 3",
			Description:    "Description 3",
			Status:         TaskStatusPending,
			Priority:       "P2",
			Payload:        []byte(`{}`),
			Dependencies:   []byte(`[]`),
			CreatedAt:      time.Now(),
			UpdatedAt:      time.Now(),
		}

		err := svc.CreateTask(ctx, task)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		err = svc.CompleteTask(ctx, "task-3")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		retrieved, err := svc.GetTask(ctx, "task-3")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if retrieved.Status != TaskStatusDone {
			t.Errorf("expected status DONE, got %v", retrieved.Status)
		}

		err = svc.FailTask(ctx, "task-3")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		retrieved, err = svc.GetTask(ctx, "task-3")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if retrieved.Status != TaskStatusFailed {
			t.Errorf("expected status FAILED, got %v", retrieved.Status)
		}
	})

	t.Run("ClaimTask_NoPendingTasks", func(t *testing.T) {
		claimed, err := svc.ClaimTask(ctx, "org-empty", "agent-1")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if claimed != nil {
			t.Fatalf("expected no task claimed, got %v", claimed)
		}
	})
}
