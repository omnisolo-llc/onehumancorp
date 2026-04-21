package repositories

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestKairosRepository(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	repo := NewKairosRepository(provider)

	provider.Exec(ctx, `
	CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_plan_id TEXT,
		title TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS kairos_state_transitions (
		id TEXT PRIMARY KEY,
		task_id TEXT,
		from_state TEXT NOT NULL,
		to_state TEXT NOT NULL,
		agent_id TEXT NOT NULL,
		reason TEXT,
		occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		worker_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS autodream_vector_memories (
		id TEXT PRIMARY KEY,
		source_mission_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`)

	t.Run("CreateSharedTask", func(t *testing.T) {
		task := &KairosSharedTask{
			ID:             "task-1",
			OrganizationID: "org-1",
			Title:          "Test Task",
			Status:         "PENDING",
		}
		err := repo.CreateSharedTask(ctx, task)
		if err != nil {
			t.Errorf("expected no error, got %v", err)
		}
	})

	t.Run("CreateStateTransition", func(t *testing.T) {
		transition := &KairosStateTransition{
			ID:        "trans-1",
			TaskID:    "task-1",
			FromState: "PENDING",
			ToState:   "IN_PROGRESS",
			AgentID:   "agent-1",
		}
		err := repo.CreateStateTransition(ctx, transition)
		if err != nil {
			t.Errorf("expected no error, got %v", err)
		}
	})

	t.Run("CreateSubAgentJob", func(t *testing.T) {
		job := &KairosSubAgentJob{
			ID:             "job-1",
			OrganizationID: "org-1",
			Payload:        json.RawMessage(`{"key": "value"}`),
			Status:         "QUEUED",
		}
		err := repo.CreateSubAgentJob(ctx, job)
		if err != nil {
			t.Errorf("expected no error, got %v", err)
		}
	})

	t.Run("CreateVectorMemory", func(t *testing.T) {
		memory := &AutodreamVectorMemory{
			ID:        "mem-1",
			Content:   "Test memory content",
			Embedding: "[]",
		}
		err := repo.CreateVectorMemory(ctx, memory)
		if err != nil {
			t.Errorf("expected no error, got %v", err)
		}
	})
}
