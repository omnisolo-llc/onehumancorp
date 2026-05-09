package interop

import (
	"context"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestRedisLock(t *testing.T) {
	s, err := miniredis.Run()
	require.NoError(t, err)
	defer s.Close()

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})
	lock := NewRedisLock(client)
	ctx := context.Background()
	resource := "test_resource"

	// Test 1: Acquire new lock
	acquired, err := lock.AcquireLock(ctx, resource, "owner1", 10)
	require.NoError(t, err)
	assert.True(t, acquired)

	// Test 2: Other owner cannot acquire
	acquired, err = lock.AcquireLock(ctx, resource, "owner2", 10)
	require.NoError(t, err)
	assert.False(t, acquired)

	// Test 3: Same owner can extend/re-acquire lock
	acquired, err = lock.AcquireLock(ctx, resource, "owner1", 20)
	require.NoError(t, err)
	assert.True(t, acquired)

	// Test 4: Release lock
	err = lock.ReleaseLock(ctx, resource, "owner1")
	require.NoError(t, err)

	// Test 5: Other owner can acquire now
	acquired, err = lock.AcquireLock(ctx, resource, "owner2", 10)
	require.NoError(t, err)
	assert.True(t, acquired)
}
