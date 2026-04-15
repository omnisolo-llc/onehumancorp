package repositories

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/dbtest"
	"github.com/onehumancorp/mono/srcs/server/db/models"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDB(t *testing.T) db.Provider {
	provider := dbtest.NewTestProvider(t)

	// Create tables for test
	ctx := context.Background()
	queries := []string{
		`CREATE TABLE IF NOT EXISTS swarm_tasks (
			id VARCHAR PRIMARY KEY,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			priority VARCHAR,
			agent_id VARCHAR,
			created_at TIMESTAMP,
			updated_at TIMESTAMP
		);`,
		`CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id VARCHAR PRIMARY KEY,
			task_id VARCHAR,
			from_state VARCHAR,
			to_state VARCHAR,
			triggered_by VARCHAR,
			transitioned_at TIMESTAMP
		);`,
		`CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id VARCHAR NOT NULL,
			depends_on_task_id VARCHAR NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);`,
	}

	for _, q := range queries {
		_, err := provider.Exec(ctx, q)
		require.NoError(t, err)
	}

	return provider
}

func TestTaskRepository(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	repo := NewTaskRepository(provider)
	ctx := context.Background()

	t.Run("Create and Get Pending", func(t *testing.T) {
		task := &models.SwarmTask{
			Title:       "Test Task",
			Description: "Test Description",
		}

		err := repo.CreateTask(ctx, task)
		require.NoError(t, err)
		assert.NotEmpty(t, task.ID)

		pending, err := repo.GetPendingTasks(ctx)
		require.NoError(t, err)
		require.Len(t, pending, 1)
		assert.Equal(t, task.Title, pending[0].Title)
	})

	t.Run("Claim Task", func(t *testing.T) {
		task := &models.SwarmTask{
			Title: "Claim Me",
		}
		err := repo.CreateTask(ctx, task)
		require.NoError(t, err)

		agentID := "agent-123"
		err = repo.ClaimTask(ctx, task.ID, agentID)
		require.NoError(t, err)

		// Ensure it's not in pending
		pending, err := repo.GetPendingTasks(ctx)
		require.NoError(t, err)
		for _, p := range pending {
			assert.NotEqual(t, task.ID, p.ID)
		}
	})

	t.Run("Complete Task", func(t *testing.T) {
		task := &models.SwarmTask{
			Title: "Complete Me",
		}
		err := repo.CreateTask(ctx, task)
		require.NoError(t, err)

		err = repo.ClaimTask(ctx, task.ID, "agent-123")
		require.NoError(t, err)

		err = repo.CompleteTask(ctx, task.ID)
		require.NoError(t, err)
	})

	t.Run("Dependencies", func(t *testing.T) {
		task1 := &models.SwarmTask{Title: "Task 1"}
		task2 := &models.SwarmTask{Title: "Task 2"}

		repo.CreateTask(ctx, task1)
		repo.CreateTask(ctx, task2)

		err := repo.AddDependency(ctx, task1.ID, task2.ID)
		require.NoError(t, err)

		deps, err := repo.GetTaskDependencies(ctx, task1.ID)
		require.NoError(t, err)
		require.Len(t, deps, 1)
		assert.Equal(t, task2.ID, deps[0])
	})
}
