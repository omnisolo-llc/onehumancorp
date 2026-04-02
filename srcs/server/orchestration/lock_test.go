package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func TestDistributedLockManager_Standalone(t *testing.T) {
	ctx := context.Background()
	dlm := NewDistributedLockManager(nil, nil)

	key := "test-lock"
	owner1 := "agent-1"
	owner2 := "agent-2"

	// Acquire lock
	acquired, err := dlm.AcquireLock(ctx, key, owner1, 1*time.Second)
	assert.NoError(t, err)
	assert.True(t, acquired)

	// Try to acquire again with different owner
	acquired2, err := dlm.AcquireLock(ctx, key, owner2, 1*time.Second)
	assert.NoError(t, err)
	assert.False(t, acquired2)

	// Release lock
	err = dlm.ReleaseLock(ctx, key, owner1)
	assert.NoError(t, err)

	// Try to acquire again
	acquired3, err := dlm.AcquireLock(ctx, key, owner2, 1*time.Second)
	assert.NoError(t, err)
	assert.True(t, acquired3)

	// Wait for expiry
	time.Sleep(1500 * time.Millisecond)

	// Acquire after expiry
	acquired4, err := dlm.AcquireLock(ctx, key, owner1, 1*time.Second)
	assert.NoError(t, err)
	assert.True(t, acquired4)
}

func TestDistributedLockManager_Postgres(t *testing.T) {
	// Let's create an integration test
	ctx := context.Background()
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	sqliteProvider, err := db.NewProvider(ctx)
	assert.NoError(t, err)

	dlm := NewDistributedLockManager(sqliteProvider, nil)

	key := "db-lock"
	acquired, err := dlm.AcquireLock(ctx, key, "agent-1", 1*time.Second)
	assert.NoError(t, err)
	assert.True(t, acquired)
}
