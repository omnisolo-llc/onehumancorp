package interop

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
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

func TestMemoryLock_UnlockRaceConditionAndFailures(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()
	lock2, _ := NewDistributedLock() // Simulate another process

	ctx := context.Background()
	key := "test_lock_key_race"

	// lock1 acquires lock
	locked, _ := lock1.Lock(ctx, key, 1*time.Second)
	if !locked {
		t.Fatalf("Expected lock1 to acquire lock")
	}

	// lock2 fails
	locked2, _ := lock2.Lock(ctx, key, 1*time.Second)
	if locked2 {
		t.Fatalf("Expected lock2 to fail")
	}

	// lock2 tries to unlock lock1's lock (should be no-op/silent success)
	_ = lock2.Unlock(ctx, key)

	// lock1 should still hold the lock
	locked3, _ := lock2.Lock(ctx, key, 1*time.Second)
	if locked3 {
		t.Fatalf("Expected lock2 to still fail because lock1 holds it")
	}

	// Wait for expiration
	time.Sleep(1500 * time.Millisecond)

	// lock2 should now acquire the lock because it's expired
	locked4, _ := lock2.Lock(ctx, key, 1*time.Second)
	if !locked4 {
		t.Fatalf("Expected lock2 to acquire lock after expiration")
	}

	_ = lock1.Unlock(ctx, key)
	_ = lock2.Unlock(ctx, key)
}

func TestMemoryLock_FailuresAndPaths(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()

	ctx := context.Background()
	key := "test_lock_key_failures"

	// lock1 acquires lock
	locked, _ := lock1.Lock(ctx, key, 1*time.Second)
	if !locked {
		t.Fatalf("Expected lock1 to acquire lock")
	}

	// Now let's try to acquire the same lock with a different token
	lock2, _ := NewDistributedLock()
	locked2, _ := lock2.Lock(ctx, key, 1*time.Second)
	if locked2 {
		t.Fatalf("Expected lock2 to fail since lock1 has it")
	}

	// Simulate invalid file content for the lock token file by overwriting it
	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)
	metaPath := filepath.Join(path, "meta.txt")
	os.WriteFile(metaPath, []byte("invalid_format"), 0666)

	// Attempt to lock again. Since it's invalid format, parsing expiry should fail, and we shouldn't get the lock
	locked3, _ := lock2.Lock(ctx, key, 1*time.Second)
	if locked3 {
		t.Fatalf("Expected lock2 to fail on invalid format lock file")
	}

	// Unlock with invalid format file should gracefully fail/ignore
	err := lock1.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Expected unlock to handle invalid format without error: %v", err)
	}

	// Since lock wasn't deleted by unlock (because of format mismatch), we delete it manually
	os.RemoveAll(path)
}

func TestMemoryLock_ExpiredLockOverwrite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()
	ctx := context.Background()
	key := "test_lock_key_expired"

	// Create an artificially expired lock file
	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)
	os.Mkdir(path, 0777)
	metaPath := filepath.Join(path, "meta.txt")

	expiry := time.Now().Add(-1 * time.Hour).Format(time.RFC3339Nano)
	os.WriteFile(metaPath, []byte(expiry+",old_token"), 0666)

	// Now try to lock
	locked, _ := lock1.Lock(ctx, key, 1*time.Second)
	if !locked {
		t.Fatalf("Expected lock1 to acquire lock by overwriting expired lock")
	}

	_ = lock1.Unlock(ctx, key)
}

func TestMemoryLock_CoveragePaths(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()
	ctx := context.Background()
	key := "test_lock_key_coverage"

	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	// Clean up any existing state
	os.RemoveAll(path)

	// Force os.Mkdir to fail with an error other than IsExist.
	// We created a file, so Mkdir fails with EEXIST.
	// Then it stats the path, sees it's not a directory, deletes it, and retries Mkdir.
	// So it should ACTUALLY ACQUIRE THE LOCK. This tests the EEXIST recovery path.
	os.WriteFile(path, []byte("file_not_dir"), 0666)

	locked, err := lock1.Lock(ctx, key, 1*time.Second)
	if !locked || err != nil {
		t.Fatalf("Expected lock to recover and succeed when a file blocked the path")
	}

	os.RemoveAll(path) // Cleanup

	// Test unlock for non-existent file
	err = lock1.Unlock(ctx, "non_existent_key")
	if err != nil {
		t.Fatalf("Expected unlock to gracefully handle non-existent file")
	}

	os.RemoveAll(path)

	// Lock it normally to test close/write errors
	locked, err = lock1.Lock(ctx, key, 1*time.Second)
	if !locked || err != nil {
		t.Fatalf("Failed to acquire lock")
	}

	// Try to unlock with a different token explicitly (simulate race condition correctly)
	lock2, _ := NewDistributedLock()
	err = lock2.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Expected unlock from different lock instance to gracefully ignore")
	}

	// Explicitly unlock it correctly
	lock1.Unlock(ctx, key)
}

func TestMemoryLock_CoveragePaths_RenameError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()
	ctx := context.Background()
	key := "test_lock_key_rename_error"

	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	// Clean up any existing state
	os.RemoveAll(path)

	locked, err := lock1.Lock(ctx, key, 1*time.Second)
	if !locked || err != nil {
		t.Fatalf("Failed to acquire lock")
	}

	// Explicitly delete the file so rename fails in Unlock
	os.RemoveAll(path)

	// Unlock should gracefully handle the rename failure
	err = lock1.Unlock(ctx, key)
	if err != nil {
		t.Fatalf("Expected unlock to gracefully handle missing file / rename error")
	}
}

func TestMemoryLock_IsExistError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	lock1, _ := NewDistributedLock()
	ctx := context.Background()
	key := "test_lock_key_is_exist"

	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	// Clean up any existing state
	os.RemoveAll(path)

	locked, err := lock1.Lock(ctx, key, 1*time.Second)
	if !locked || err != nil {
		t.Fatalf("Failed to acquire lock")
	}

	// Attempt to acquire the same lock -> returns os.IsExist internally but false, nil to the caller
	locked2, err2 := lock1.Lock(ctx, key, 1*time.Second)
	if locked2 || err2 != nil {
		t.Fatalf("Expected lock to fail gracefully")
	}

	os.Remove(path) // Cleanup
}

func TestCloudLock(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer s.Close()

	os.Setenv("REDIS_URL", "redis://"+s.Addr())
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("REDIS_URL")
	defer os.Unsetenv("OHC_STANDALONE")

	lock, err := NewDistributedLock()
	if err != nil {
		t.Fatalf("Failed to create cloud lock: %v", err)
	}

	ctx := context.Background()
	key := "test_cloud_lock_key"

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
}
