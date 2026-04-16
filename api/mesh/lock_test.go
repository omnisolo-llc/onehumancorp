package mesh

import (
	"context"
	"testing"
	"time"
)

func TestDistributedLock(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Ensure Redis is reachable, otherwise skip the test.
	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	lock := NewDistributedLock(client, "test-resource")

	// Acquire lock
	err := lock.Acquire(ctx, 1*time.Second)
	if err != nil {
	    t.Fatalf("Failed to acquire lock: %v", err)
	}

	// Attempt to acquire again (should block, so we use a timeout context)
	ctxTimeout, cancelTimeout := context.WithTimeout(ctx, 200*time.Millisecond)
	defer cancelTimeout()
	err = lock.Acquire(ctxTimeout, 1*time.Second)
	if err != context.DeadlineExceeded {
	    t.Fatalf("Expected DeadlineExceeded, got: %v", err)
	}

	// Release lock
	err = lock.Release(ctx)
	if err != nil {
	    t.Fatalf("Failed to release lock: %v", err)
	}

	// Acquire again
	err = lock.Acquire(ctx, 1*time.Second)
	if err != nil {
	    t.Fatalf("Failed to acquire lock after release: %v", err)
	}
}
