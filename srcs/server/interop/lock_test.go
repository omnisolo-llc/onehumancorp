package interop

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestFileLock(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Failed to create file lock: %v", err)
	}

	ctx := context.Background()
	key := "test_lock_key"

	// 1. Initial lock should succeed
	locked, err := lock.Lock(ctx, key, 3*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked {
		t.Errorf("Expected to acquire lock, but failed")
	}

	// 2. Second lock should fail
	locked2, err := lock.Lock(ctx, key, 3*time.Second)
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
	locked3, err := lock.Lock(ctx, key, 3*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked3 {
		t.Errorf("Expected to acquire lock after unlock, but failed")
	}

    err = lock.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Unlocking returned error: %v", err)
	}

	// 5. Test expiration
    // Atomic locks using os.O_EXCL only release when os.Remove is called.
    // In actual use, if a process dies before Unlock, the lock remains stale until the TTL is reached.
    // We simulate a stale lock by creating a file and updating its timestamp.

    // Acquire
    locked4, err := lock.Lock(ctx, key, 3*time.Second)
    if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
    if !locked4 {
        t.Fatalf("Expected to acquire lock")
    }

    // Mock expiration
    if fl, ok := lock.(*fileLock); ok {
        fl.mu.Lock()
        if fl.locks[key] {
            delete(fl.locks, key)
            // Push ModTime back
            lockFile := filepath.Join(fl.baseDir, key+".lock")
            past := time.Now().Add(-5 * time.Second)
            os.Chtimes(lockFile, past, past)
        }
        fl.mu.Unlock()
    }

	locked5, err := lock.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked5 {
		t.Errorf("Expected to acquire lock after expiration, but failed")
	}
    lock.Unlock(ctx, key)
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

    if _, ok := lock.(*fileLock); !ok {
        t.Errorf("Expected fallback to fileLock, got %T", lock)
    }
}
