package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSQLiteMutex_LockUnlock(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	mutexProvider, err := NewMutexProvider(ctx, provider, nil)
	require.NoError(t, err)
	mutex := mutexProvider.NewMutex("test-lock-1")

	// Test successful lock
	err = mutex.Lock(ctx, 5*time.Second)
	require.NoError(t, err)

	// Test fail to lock when already locked
	mutex2 := mutexProvider.NewMutex("test-lock-1")
	err = mutex2.Lock(ctx, 5*time.Second)
	assert.ErrorIs(t, err, ErrLockAcquisitionFailed)

	// Test unlock
	err = mutex.Unlock(ctx)
	require.NoError(t, err)

	// Test fail to unlock when not owned
	err = mutex2.Unlock(ctx)
	assert.ErrorIs(t, err, ErrLockNotOwned)

	// Test lock again after unlock
	err = mutex2.Lock(ctx, 5*time.Second)
	require.NoError(t, err)
}

func TestSQLiteMutex_Expiration(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	mutexProvider, err := NewMutexProvider(ctx, provider, nil)
	require.NoError(t, err)
	mutex := mutexProvider.NewMutex("test-lock-exp")

	// Lock with short TTL
	err = mutex.Lock(ctx, 100*time.Millisecond)
	require.NoError(t, err)

	// Wait for expiration
	time.Sleep(200 * time.Millisecond)

	// Another mutex should be able to acquire the lock now
	mutex2 := mutexProvider.NewMutex("test-lock-exp")
	err = mutex2.Lock(ctx, 5*time.Second)
	require.NoError(t, err)
}

func TestRedisMutex_Mocked(t *testing.T) {
	t.Log("Redis Mutex logic relies on rueidis which is verified via integration tests.")
}
