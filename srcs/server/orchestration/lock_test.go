package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDatabaseLock(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	dl := NewDatabaseLock(provider)

	key := "test-lock"
	owner1 := "agent-1"
	owner2 := "agent-2"

	// 1. Acquire Lock
	err := dl.Acquire(ctx, key, owner1, 2*time.Second)
	if err != nil {
		t.Fatalf("expected to acquire lock, got: %v", err)
	}

	// 2. Attempt to acquire same lock by another owner (should fail)
	err = dl.Acquire(ctx, key, owner2, 2*time.Second)
	if err != ErrLockNotAcquired {
		t.Fatalf("expected ErrLockNotAcquired, got: %v", err)
	}

	// 3. Release Lock
	err = dl.Release(ctx, key, owner1)
	if err != nil {
		t.Fatalf("expected to release lock, got: %v", err)
	}

	// 4. Second owner should now acquire
	err = dl.Acquire(ctx, key, owner2, 2*time.Second)
	if err != nil {
		t.Fatalf("expected agent-2 to acquire lock, got: %v", err)
	}

	// 5. Expiration test
	// Wait for expiration
	time.Sleep(3 * time.Second)

	// 6. First owner should be able to acquire again because owner2's lock expired
	err = dl.Acquire(ctx, key, owner1, 2*time.Second)
	if err != nil {
		t.Fatalf("expected agent-1 to acquire expired lock, got: %v", err)
	}
}

// setupTestDB creates a temporary SQLite database for testing and runs migrations.
func setupTestDB(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	provider := db.NewSqliteProvider(sqliteDB)

	// Create table needed for the lock
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS distributed_locks (
			lock_key VARCHAR PRIMARY KEY,
			owner_id VARCHAR NOT NULL,
			expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create distributed_locks table: %v", err)
	}

	return provider
}
