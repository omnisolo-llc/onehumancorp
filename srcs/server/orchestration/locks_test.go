package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDistributedLockProvider_SQLite(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Ensure the table exists
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS distributed_locks (
			lock_key TEXT PRIMARY KEY,
			owner_id TEXT NOT NULL,
			expires_at DATETIME NOT NULL
		)
	`)
	require.NoError(t, err)

	lockProvider, err := NewDistributedLockProvider(ctx, provider, nil)
	require.NoError(t, err)

	lock := lockProvider.NewLock("test-lock")

	err = lock.Lock(ctx, 5*time.Second)
	assert.NoError(t, err, "First lock should succeed")

	// Try acquiring again with a new lock object (should fail)
	lock2 := lockProvider.NewLock("test-lock")
	err = lock2.Lock(ctx, 5*time.Second)
	assert.Error(t, err, "Second lock should fail")
	assert.Equal(t, ErrLockAcquisitionFailed, err)

	err = lock.Unlock(ctx)
	assert.NoError(t, err, "Unlock should succeed")

	// Now lock2 should succeed
	err = lock2.Lock(ctx, 5*time.Second)
	assert.NoError(t, err, "Second lock should succeed after unlock")
}
