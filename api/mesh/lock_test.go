package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDistributedLock(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx := context.Background()
	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable: %v", err)
	}

	lock := NewDistributedLock(client, "test_lock")

	// Acquire
	err := lock.Acquire(ctx, 5*time.Second)
	require.NoError(t, err)

	// Attempt second acquire should timeout/block, so we use context with timeout
	ctxTimeout, cancel := context.WithTimeout(ctx, 500*time.Millisecond)
	defer cancel()
	lock2 := NewDistributedLock(client, "test_lock")
	err = lock2.Acquire(ctxTimeout, 5*time.Second)
	assert.ErrorIs(t, err, context.DeadlineExceeded)

	// Release
	err = lock.Release(ctx)
	require.NoError(t, err)

	// Now second lock can acquire
	err = lock2.Acquire(ctx, 5*time.Second)
	require.NoError(t, err)
	lock2.Release(ctx)
}

func TestDistributedLock_AcquireWithRedlock(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx := context.Background()
	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable: %v", err)
	}

	// For test, we use just 1 node simulated as multi nodes to test logic
	// We CANNOT use client multiple times because SETNX returns true only once
	nodes := []*redis.Client{client}

	lock := NewDistributedLock(client, "test_redlock")

	// Acquire Redlock
	err := lock.AcquireWithRedlock(ctx, 5*time.Second, nodes)
	require.NoError(t, err)

	// Attempt second acquire should fail
	ctxTimeout, cancel := context.WithTimeout(ctx, 500*time.Millisecond)
	defer cancel()
	lock2 := NewDistributedLock(client, "test_redlock")
	err = lock2.AcquireWithRedlock(ctxTimeout, 5*time.Second, nodes)
	assert.ErrorIs(t, err, context.DeadlineExceeded)

	// Release Redlock
	err = lock.ReleaseRedlock(ctx, nodes)
	require.NoError(t, err)
}
