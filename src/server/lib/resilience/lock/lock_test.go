package lock

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestDatabaseLockProvider_SQLite(t *testing.T) {
	testDB := db.NewTestProvider(t)
	defer testDB.Close()

	_, err := testDB.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS distributed_locks (
			key VARCHAR(255) PRIMARY KEY,
			token VARCHAR(255) NOT NULL,
			expires_at TIMESTAMP NOT NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	provider := NewDatabaseLockProvider(testDB)
	ctx := context.Background()

	t.Run("acquire new lock", func(t *testing.T) {
		locked, unlock, err := provider.TryLock(ctx, "test-key-1", 5*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected to acquire lock, but didn't")
		}
		if unlock == nil {
			t.Fatalf("Expected unlock function, got nil")
		}

		// Try to acquire again, should fail
		locked2, _, err2 := provider.TryLock(ctx, "test-key-1", 5*time.Second)
		if err2 != nil {
			t.Fatalf("Expected no error on second lock attempt, got %v", err2)
		}
		if locked2 {
			t.Fatalf("Expected NOT to acquire lock second time")
		}

		// Unlock
		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on unlock, got %v", err)
		}

		// Try again after unlock, should succeed
		locked3, unlock3, err3 := provider.TryLock(ctx, "test-key-1", 5*time.Second)
		if err3 != nil {
			t.Fatalf("Expected no error on third lock attempt, got %v", err3)
		}
		if !locked3 {
			t.Fatalf("Expected to acquire lock after unlock")
		}

		// Cleanup
		_ = unlock3(ctx)
	})

	t.Run("acquire expired lock", func(t *testing.T) {
		// Acquire with 1s TTL
		locked, unlock, err := provider.TryLock(ctx, "test-key-expired", 1*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected to acquire lock")
		}

		// Wait for expiration
		time.Sleep(1500 * time.Millisecond)

		// Try to acquire again, should succeed because it expired
		locked2, unlock2, err2 := provider.TryLock(ctx, "test-key-expired", 5*time.Second)
		if err2 != nil {
			t.Fatalf("Expected no error, got %v", err2)
		}
		if !locked2 {
			t.Fatalf("Expected to acquire expired lock")
		}

		// Unlocking first lock should not delete the second lock
		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on old unlock, got %v", err)
		}

		// Try to acquire again, should fail because second lock is active
		locked3, _, err3 := provider.TryLock(ctx, "test-key-expired", 5*time.Second)
		if err3 != nil {
			t.Fatalf("Expected no error, got %v", err3)
		}
		if locked3 {
			t.Fatalf("Expected NOT to acquire lock since active lock should not be deleted by old unlock")
		}

		defer unlock2(ctx)
	})
}
