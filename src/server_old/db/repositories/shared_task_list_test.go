package repositories

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
	_ "modernc.org/sqlite"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupSharedTaskTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	// Create tables for test
	_, err = sqlDB.Exec(`CREATE TABLE shared_tasks_v4 (
		id VARCHAR PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		title VARCHAR NOT NULL,
		description TEXT,
		status VARCHAR NOT NULL DEFAULT 'PENDING',
		agent_id VARCHAR,
		priority VARCHAR NOT NULL DEFAULT 'P2',
		payload TEXT,
		parent_plan_id TEXT,
		dependencies TEXT NOT NULL DEFAULT '[]',
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	require.NoError(t, err)

	_, err = sqlDB.Exec(`CREATE TABLE sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		worker_id TEXT,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	require.NoError(t, err)

	_, err = sqlDB.Exec(`CREATE TABLE state_machine_transitions (
		id TEXT PRIMARY KEY,
		entity_id TEXT NOT NULL,
		entity_type TEXT NOT NULL,
		from_state TEXT NOT NULL,
		to_state TEXT NOT NULL,
		agent_id TEXT,
		reason TEXT,
		occurred_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	require.NoError(t, err)

	t.Cleanup(func() {
		sqlDB.Close()
	})

	return db.NewSqliteProvider(sqlDB)
}

func TestSharedTaskListRepository(t *testing.T) {
	provider := setupSharedTaskTestDB(t)
	repo := NewSharedTaskListRepository(provider, nil)
	ctx := context.Background()

	t.Run("Create and Get Task", func(t *testing.T) {
		task := &models.SharedTaskV4{
			ID:             "task-1",
			OrganizationID: "org-1",
			Title:          "Test Task",
			Status:         "PENDING",
			Priority:       "P0",
			Dependencies:   "[]",
		}
		err := repo.CreateTask(ctx, task)
		assert.NoError(t, err)

		got, err := repo.GetTask(ctx, "task-1")
		assert.NoError(t, err)
		assert.NotNil(t, got)
		assert.Equal(t, task.Title, got.Title)
		assert.Equal(t, "PENDING", got.Status)
	})

	t.Run("DAG Enforcement", func(t *testing.T) {
		// task-2 depends on task-1 (which is currently PENDING/ASSIGNED, not COMPLETED)
		task2 := &models.SharedTaskV4{
			ID:             "task-2",
			OrganizationID: "org-1",
			Title:          "Task 2",
			Status:         "PENDING",
			Priority:       "P0",
			Dependencies:   "[\"task-1\"]",
		}
		err := repo.CreateTask(ctx, task2)
		assert.NoError(t, err)

		// Claim should NOT return task-2 because task-1 is not COMPLETED
		claimed, err := repo.ClaimTask(ctx, "org-1", "agent-1")
		assert.NoError(t, err)
		if claimed != nil {
			assert.NotEqual(t, "task-2", claimed.ID)
		}

		// Complete task-1
		err = repo.UpdateTaskStatus(ctx, "task-1", "PENDING", "COMPLETED", "agent-0", "done")
		if err != nil {
			// maybe it was assigned
			err = repo.UpdateTaskStatus(ctx, "task-1", "ASSIGNED", "COMPLETED", "agent-0", "done")
		}
		assert.NoError(t, err)

		// Now Claim should return task-2
		claimed, err = repo.ClaimTask(ctx, "org-1", "agent-1")
		assert.NoError(t, err)
		assert.NotNil(t, claimed)
		assert.Equal(t, "task-2", claimed.ID)
	})

	t.Run("Update Status", func(t *testing.T) {
		err := repo.UpdateTaskStatus(ctx, "task-2", "ASSIGNED", "IN_PROGRESS", "agent-1", "Starting work")
		assert.NoError(t, err)

		got, _ := repo.GetTask(ctx, "task-2")
		assert.Equal(t, "IN_PROGRESS", got.Status)

		// Verify audit log
		var count int
		err = provider.QueryRow(ctx, "SELECT count(*) FROM state_machine_transitions WHERE entity_id = 'task-2' AND entity_type = 'SHARED_TASK'").Scan(&count)
		assert.NoError(t, err)
		assert.GreaterOrEqual(t, count, 2) // Claim + Update
	})
}

func TestSubAgentQueueRepository(t *testing.T) {
	provider := setupSharedTaskTestDB(t)
	repo := NewSubAgentQueueRepository(provider)
	ctx := context.Background()

	t.Run("Enqueue and Claim Job", func(t *testing.T) {
		job := &models.SubAgentJob{
			ID:             "job-1",
			OrganizationID: "org-1",
			ParentTaskID:   "task-1",
			Status:         "PENDING",
		}
		err := repo.Enqueue(ctx, job)
		assert.NoError(t, err)

		claimed, err := repo.ClaimJob(ctx, "org-1", "worker-1")
		assert.NoError(t, err)
		assert.NotNil(t, claimed)
		assert.Equal(t, "job-1", claimed.ID)
		assert.Equal(t, "IN_PROGRESS", claimed.Status)
		assert.Equal(t, "worker-1", *claimed.WorkerID)

		// Verify audit log for job
		var count int
		err = provider.QueryRow(ctx, "SELECT count(*) FROM state_machine_transitions WHERE entity_id = 'job-1' AND entity_type = 'SUB_AGENT_JOB'").Scan(&count)
		assert.NoError(t, err)
		assert.Equal(t, 1, count)
	})

	t.Run("Update Job Status", func(t *testing.T) {
		err := repo.UpdateJobStatus(ctx, "job-1", "COMPLETED", "worker-1", "Finished")
		assert.NoError(t, err)

		var status string
		err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_queue WHERE id = 'job-1'").Scan(&status)
		assert.NoError(t, err)
		assert.Equal(t, "COMPLETED", status)

		// Verify audit log
		var count int
		err = provider.QueryRow(ctx, "SELECT count(*) FROM state_machine_transitions WHERE entity_id = 'job-1' AND entity_type = 'SUB_AGENT_JOB'").Scan(&count)
		assert.NoError(t, err)
		assert.Equal(t, 2, count)
	})
}
