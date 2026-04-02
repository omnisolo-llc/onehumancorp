package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func setupTestDB(t *testing.T) db.Provider {
	provider, err := db.NewSqliteProviderMemory()
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	assert.NoError(t, err)
	return provider
}

func setupRedis(t *testing.T) (*miniredis.Miniredis, *redis.Client) {
	mr, err := miniredis.Run()
	assert.NoError(t, err)

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	return mr, client
}

func TestTaskManager_Standalone(t *testing.T) {
	os.Unsetenv("OHC_MULTITENANT")
	provider := setupTestDB(t)
	defer provider.Close()

	tm := NewTaskManager(provider, nil)
	ctx := context.Background()

	// Add Task
	task, err := tm.AddTask(ctx, "m1", "Test Title", "Test Desc", "P1")
	assert.NoError(t, err)
	assert.NotNil(t, task)
	assert.Equal(t, "PENDING", task.Status)

	// Claim Task
	claimed, err := tm.ClaimTask(ctx, "agent1")
	assert.NoError(t, err)
	assert.NotNil(t, claimed)
	assert.Equal(t, "IN_PROGRESS", claimed.Status)
	assert.Equal(t, "agent1", claimed.AssignedAgentID)
	assert.Equal(t, task.ID, claimed.ID)

	// Complete Task
	err = tm.CompleteTask(ctx, claimed.ID)
	assert.NoError(t, err)

	// Claim Empty
	_, err = tm.ClaimTask(ctx, "agent2")
	assert.Error(t, err)
	assert.Equal(t, "no pending tasks", err.Error())
}

func TestTaskManager_Cloud(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := setupTestDB(t)
	defer provider.Close()

	mr, redisClient := setupRedis(t)
	defer mr.Close()

	tm := NewTaskManager(provider, redisClient)
	ctx := context.Background()

	// Add Task
	task, err := tm.AddTask(ctx, "m1", "Cloud Title", "Cloud Desc", "P0")
	assert.NoError(t, err)
	assert.NotNil(t, task)

	// Claim Task
	claimed, err := tm.ClaimTask(ctx, "agentCloud")
	assert.NoError(t, err)
	assert.NotNil(t, claimed)
	assert.Equal(t, "IN_PROGRESS", claimed.Status)
	assert.Equal(t, "agentCloud", claimed.AssignedAgentID)

	// Check lock was set and cleared or handle properly
	// Since the lock is set and kept for 10s or whatever, it might be present in miniredis
	mr.FastForward(11 * time.Second)

	// Add another task
	_, _ = tm.AddTask(ctx, "m2", "Cloud Title 2", "Cloud Desc 2", "P2")

	// Simulate lock failure
	mr.Set("task_lock:"+claimed.ID, "otherAgent") // mock the new task getting locked (assuming IDs)
	// Just claim again and it should work for m2
	claimed2, err := tm.ClaimTask(ctx, "agentCloud2")
	assert.NoError(t, err)
	assert.NotNil(t, claimed2)
	assert.NotEqual(t, claimed.ID, claimed2.ID)
}
