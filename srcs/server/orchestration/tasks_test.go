package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestTaskManager(t *testing.T) {
	ctx := context.Background()
	t.Setenv("DATABASE_URL", "sqlite://:memory:")

	testDB, err := db.New(ctx)
	require.NoError(t, err)
	err = testDB.RunMigrations(ctx)
	require.NoError(t, err)

	tm := NewTaskManager(testDB)

	// Test claiming when no tasks exist
	task, err := tm.ClaimTask(ctx, "agent-1")
	require.NoError(t, err)
	assert.Nil(t, task)

	// Create a task
	createdTask, err := tm.CreateTask(ctx, "mission-123", "Test Task", "Test Desc", "P1")
	require.NoError(t, err)
	assert.NotNil(t, createdTask)
	assert.Equal(t, "PENDING", createdTask.Status)

	// Claim the task
	claimedTask, err := tm.ClaimTask(ctx, "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, createdTask.ID, claimedTask.ID)
	assert.Equal(t, "IN_PROGRESS", claimedTask.Status)
	assert.Equal(t, "agent-1", claimedTask.AssignedAgentID)

	// Try claiming again, should be empty
	task2, err := tm.ClaimTask(ctx, "agent-2")
	require.NoError(t, err)
	assert.Nil(t, task2)
}
