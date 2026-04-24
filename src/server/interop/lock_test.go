package interop

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestMemoryLock(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Failed to create memory lock: %v", err)
	}

	ctx := context.Background()
	key := "test_lock_key"

	// 1. Initial lock should succeed
	locked, err := lock.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked {
		t.Errorf("Expected to acquire lock, but failed")
	}

	// 2. Second lock should fail
	locked2, err := lock.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if locked2 {
		t.Errorf("Expected to fail acquiring lock, but succeeded")
	}

	// 3. Unlock
	err = lock.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Unlocking returned error: %v", err)
	}

	// 4. Lock again should succeed
	locked3, err := lock.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked3 {
		t.Errorf("Expected to acquire lock after unlock, but failed")
	}

	// 5. Test expiration
	time.Sleep(1500 * time.Millisecond)
	locked4, err := lock.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked4 {
		t.Errorf("Expected to acquire lock after expiration, but failed")
	}
}

func TestNewDistributedLockFallback(t *testing.T) {
    // Set invalid REDIS_URL to trigger fallback
	os.Setenv("REDIS_URL", "invalid_url")
	os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("REDIS_URL")

	lock, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Expected fallback to succeed, but got error: %v", err)
	}

    if _, ok := lock.(*memoryLock); !ok {
        t.Errorf("Expected fallback to memoryLock, got %T", lock)
    }
}
