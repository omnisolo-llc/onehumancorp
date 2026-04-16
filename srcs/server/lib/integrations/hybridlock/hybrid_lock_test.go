package hybridlock

import (
    "context"
    "testing"
    "time"

    "github.com/stretchr/testify/assert"
)

func TestHybridLockManager_StandaloneMode(t *testing.T) {
    manager := NewHybridLockManager(nil)
    ctx := context.Background()

    // Test Acquire
    lock1, err := manager.Acquire(ctx, "test-key", "token-1", 100*time.Millisecond)
    assert.NoError(t, err)
    assert.NotNil(t, lock1)

    // Test Acquire failure (locked)
    lock2, err := manager.Acquire(ctx, "test-key", "token-2", 100*time.Millisecond)
    assert.NoError(t, err)
    assert.Nil(t, lock2)

    // Test Release
    err = manager.Release(ctx, lock1)
    assert.NoError(t, err)

    // Test Release with wrong token shouldn't delete another lock
    lock3, err := manager.Acquire(ctx, "test-key-2", "token-3", 100*time.Millisecond)
    assert.NoError(t, err)
    assert.NotNil(t, lock3)

    wrongLock := &Lock{key: "test-key-2", token: "wrong-token"}
    err = manager.Release(ctx, wrongLock)
    assert.NoError(t, err)

    // Original lock should still be there
    lock4, err := manager.Acquire(ctx, "test-key-2", "token-4", 100*time.Millisecond)
    assert.NoError(t, err)
    assert.Nil(t, lock4)

    // Test Expiration
    time.Sleep(150 * time.Millisecond) // Wait for expiration
    lock5, err := manager.Acquire(ctx, "test-key-2", "token-5", 100*time.Millisecond)
    assert.NoError(t, err)
    assert.NotNil(t, lock5) // Should acquire because lock3 expired
}
