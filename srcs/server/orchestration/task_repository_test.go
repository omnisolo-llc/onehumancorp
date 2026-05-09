package orchestration

import (
	"context"
	"database/sql"
	"sync"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTaskTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS ohc_tasks (
			id VARCHAR PRIMARY KEY,
			tenant_id VARCHAR NOT NULL,
			title VARCHAR,
			description TEXT,
			status VARCHAR NOT NULL,
			assigned_agent_id VARCHAR,
			priority INTEGER DEFAULT 0,
			payload JSONB,
			parent_task_id TEXT,
			workflow_state TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return db
}

func TestDBTaskRepository_ClaimTask_Success(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()
	repo := NewDBTaskRepository(db)
	ctx := context.Background()

	task := &Task{
		TenantID: "tenant-1",
		Title:    "Test Task",
		Status:   "PENDING",
	}

	err := repo.CreateTask(ctx, task)
	require.NoError(t, err)
	require.NotEmpty(t, task.ID)

	claimedTask, err := repo.ClaimTask(ctx, task.TenantID, task.ID, "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, "IN_PROGRESS", claimedTask.Status)
	require.NotNil(t, claimedTask.AssignedAgentID)
	assert.Equal(t, "agent-1", *claimedTask.AssignedAgentID)
}

func TestDBTaskRepository_ClaimTask_RaceCondition(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()
	repo := NewDBTaskRepository(db)
	ctx := context.Background()

	task := &Task{
		TenantID: "tenant-1",
		Title:    "Test Task Race",
		Status:   "PENDING",
	}

	err := repo.CreateTask(ctx, task)
	require.NoError(t, err)
	require.NotEmpty(t, task.ID)

	var wg sync.WaitGroup
	var successCount int
	var mu sync.Mutex

	// Simulate 10 agents trying to claim the same task simultaneously
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(agentID string) {
			defer wg.Done()
			claimedTask, err := repo.ClaimTask(ctx, task.TenantID, task.ID, agentID)
			if err == nil && claimedTask != nil {
				mu.Lock()
				successCount++
				mu.Unlock()
			}
		}( "agent-test")
	}

	wg.Wait()

	// Only one agent should successfully claim the task
	assert.Equal(t, 1, successCount)
}

func TestDBTaskRepository_ClaimTask_NotPending(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()
	repo := NewDBTaskRepository(db)
	ctx := context.Background()

	task := &Task{
		TenantID: "tenant-1",
		Title:    "Test Task Not Pending",
		Status:   "DONE",
	}

	err := repo.CreateTask(ctx, task)
	require.NoError(t, err)

	claimedTask, err := repo.ClaimTask(ctx, task.TenantID, task.ID, "agent-1")
	assert.Error(t, err)
	assert.Nil(t, claimedTask)
	assert.Contains(t, err.Error(), "failed to claim task")
}