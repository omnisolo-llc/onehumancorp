package lock

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type errorString string

func (e errorString) Error() string { return string(e) }

// Mock DB Provider to simulate Postgres responses to achieve 100% coverage
type mockPostgresDB struct {
	db.Provider
	queryRowFunc func(ctx context.Context, sql string, arguments ...any) db.Row
	execFunc func(ctx context.Context, sql string, arguments ...any) (int64, error)
}

func (m *mockPostgresDB) IsSQLite() bool {
	return false
}

type mockRow struct {
	scanFunc func(dest ...any) error
}

func (r *mockRow) Scan(dest ...any) error {
	if r.scanFunc != nil {
		return r.scanFunc(dest...)
	}
	return nil
}

func (m *mockPostgresDB) QueryRow(ctx context.Context, sql string, arguments ...any) db.Row {
	if m.queryRowFunc != nil {
		return m.queryRowFunc(ctx, sql, arguments...)
	}
	return &mockRow{}
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
			queryRowFunc: func(ctx context.Context, sql string, arguments ...any) db.Row {
				return &mockRow{
					scanFunc: func(dest ...any) error {
						// Return success by not returning an error
						return nil
					},
				}
			},
			execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
				return 1, nil // Unlock logic uses Exec
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
			queryRowFunc: func(ctx context.Context, sql string, arguments ...any) db.Row {
				return &mockRow{
					scanFunc: func(dest ...any) error {
						// Return pgx/sql error for no rows
						return errorString("no rows in result set")
					},
				}
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
