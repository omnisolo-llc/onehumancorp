package interop

import (
	"context"
	"testing"
	"time"
)

func TestMemoryLock_TryLock(t *testing.T) {
	ml := &memoryLock{locks: make(map[string]memoryLockItem)}
	ctx := context.Background()
	key := "test_lock"
	token := "token_1"
	otherToken := "token_2"

	// Acquire lock successfully
	err := ml.TryLock(ctx, key, token, 50*time.Millisecond)
	if err != nil {
		t.Fatalf("expected to acquire lock, got %v", err)
	}

	// Fail to acquire already held lock by someone else
	err = ml.TryLock(ctx, key, otherToken, 50*time.Millisecond)
	if err != ErrLockNotAcquired {
		t.Fatalf("expected ErrLockNotAcquired, got %v", err)
	}

	// Extend the same lock
	err = ml.TryLock(ctx, key, token, 50*time.Millisecond)
	if err != nil {
		t.Fatalf("expected to be able to extend lock, got %v", err)
	}

	// Wait for expiration and acquire with new token
	time.Sleep(60 * time.Millisecond)
	err = ml.TryLock(ctx, key, otherToken, 50*time.Millisecond)
	if err != nil {
		t.Fatalf("expected to acquire lock after expiration, got %v", err)
	}

	// Fail to unlock with wrong token
	err = ml.Unlock(ctx, key, token)
	if err != ErrLockNotHeld {
		t.Fatalf("expected ErrLockNotHeld for wrong token unlock, got %v", err)
	}

	// Unlock successfully
	err = ml.Unlock(ctx, key, otherToken)
	if err != nil {
		t.Fatalf("expected to unlock successfully, got %v", err)
	}

	// Unlock already released lock
	err = ml.Unlock(ctx, key, otherToken)
	if err != ErrLockNotHeld {
		t.Fatalf("expected ErrLockNotHeld, got %v", err)
	}
}
