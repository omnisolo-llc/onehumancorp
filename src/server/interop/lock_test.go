package interop

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"os"
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
	// Clean up after 5
	err = lock.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Unlocking returned error: %v", err)
	}
}

func TestFileLock_RaceConditionAndReclamation(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Failed to create file lock 1: %v", err)
	}

	lock2, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Failed to create file lock 2: %v", err)
	}

	ctx := context.Background()
	key := "test_race_reclaim_key"

	// lock1 acquires lock
	locked, err := lock1.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if !locked {
		t.Fatalf("Expected to acquire lock, but failed")
	}

	// simulate the race condition: process 1 has lock, but we corrupt/delete the info file
	// lock2 tries to get it and should FAIL because the directory exists but the file is missing
	h := sha256.New()
	h.Write([]byte(key))
	safeKey := hex.EncodeToString(h.Sum(nil))
	lockDir := os.TempDir() + string(os.PathSeparator) + "ohc_lock_" + safeKey
	// Find and remove the actual info file since its name is dynamic now
	entries, _ := os.ReadDir(lockDir)
	for _, entry := range entries {
		os.Remove(lockDir + string(os.PathSeparator) + entry.Name())
	}

	lockedRace, err := lock2.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Locking returned error: %v", err)
	}
	if lockedRace {
		t.Errorf("Expected to NOT acquire lock due to missing info file (assuming locked), but it succeeded")
	}

	// now we clean it up and let lock1 grab it normally, to test true reclamation
	os.RemoveAll(lockDir)

	lockedAgain, err := lock1.Lock(ctx, key, 1*time.Second)
	if err != nil || !lockedAgain {
		t.Fatalf("Lock1 failed to lock again: err=%v locked=%v", err, lockedAgain)
	}

	// Wait for expiration
	time.Sleep(1500 * time.Millisecond)

	// lock2 tries to get the expired lock - it should successfully reclaim
	lockedReclaim, err := lock2.Lock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Lock2 failed during reclaim: %v", err)
	}
	if !lockedReclaim {
		t.Errorf("Lock2 expected to successfully reclaim expired lock, but failed")
	}

	// clean up
	lock2.Unlock(ctx, key)
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
