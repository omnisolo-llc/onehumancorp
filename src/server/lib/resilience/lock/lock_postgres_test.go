package lock

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

// Mock DB Provider to simulate Postgres responses to achieve 100% coverage
type mockPostgresDB struct {
	db.Provider
	execFunc func(ctx context.Context, sql string, arguments ...any) (int64, error)
}

func (m *mockPostgresDB) IsSQLite() bool {
	return false
}

func (m *mockPostgresDB) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return 1, nil // Simulate success by default
}

func TestDatabaseLockProvider_Postgres_Coverage(t *testing.T) {
	// This test uses a mock to ensure tryPostgresLock code paths are covered

	ctx := context.Background()

	t.Run("acquire new lock", func(t *testing.T) {
		mockDB := &mockPostgresDB{
			execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
				return 1, nil // 1 row affected
			},
		}
		provider := NewDatabaseLockProvider(mockDB)

		locked, unlock, err := provider.TryLock(ctx, "pg-test", 5*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected lock to be acquired")
		}
		if unlock == nil {
			t.Fatalf("Expected unlock func, got nil")
		}

		// Test unlock
		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on unlock, got %v", err)
		}
	})

	t.Run("fail to acquire lock", func(t *testing.T) {
		mockDB := &mockPostgresDB{
			execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
				return 0, nil // 0 rows affected
			},
		}
		provider := NewDatabaseLockProvider(mockDB)

		locked, unlock, err := provider.TryLock(ctx, "pg-test", 5*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if locked {
			t.Fatalf("Expected lock NOT to be acquired")
		}
		if unlock != nil {
			t.Fatalf("Expected unlock func to be nil")
		}
	})
}
