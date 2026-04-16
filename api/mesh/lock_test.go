package mesh

import (
	"context"
	"testing"
	"time"
)

func TestDistributedLock(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	key := "test_lock"

	// Clean up any existing lock
	client.Del(ctx, "lock:"+key)

	lock1 := NewDistributedLock(client, key)
	lock2 := NewDistributedLock(client, key)

	// Test 1: Acquire lock
	err := lock1.Acquire(ctx, 5*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}

	// Test 2: Ensure lock2 blocks/fails to acquire immediately
	// Use a very short context to test blocking
	shortCtx, shortCancel := context.WithTimeout(ctx, 100*time.Millisecond)
	defer shortCancel()

	err = lock2.Acquire(shortCtx, 5*time.Second)
	if err != context.DeadlineExceeded {
		t.Fatalf("Expected context.DeadlineExceeded, got: %v", err)
	}

	// Test 3: Release lock
	err = lock1.Release(ctx)
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	// Test 4: Acquire lock after release
	err = lock2.Acquire(ctx, 5*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock after release: %v", err)
	}

	// Cleanup
	lock2.Release(ctx)
}
