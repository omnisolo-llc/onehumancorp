package lock

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

// Mock DB Provider to simulate Postgres responses to achieve 100% coverage
type mockPostgresDB struct {
	db.Provider
	execFunc     func(ctx context.Context, sql string, arguments ...any) (int64, error)
	queryRowFunc func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row
}

type mockRow struct {
	scanFunc func(dest ...any) error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.scanFunc != nil {
		return m.scanFunc(dest...)
	}
	return nil
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

func (m *mockPostgresDB) QueryRow(ctx context.Context, sql string, arguments ...any) db.Row {
	if m.queryRowFunc != nil {
		return m.queryRowFunc(ctx, sql, arguments...)
	}
	return &mockRow{}
}

func TestDatabaseLockProvider_Postgres_Coverage(t *testing.T) {
	// This test uses a mock to ensure tryPostgresLock code paths are covered

	ctx := context.Background()

	t.Run("acquire new lock", func(t *testing.T) {
		mockDB := &mockPostgresDB{
			queryRowFunc: func(ctx context.Context, sql string, arguments ...any) db.Row {
				return &mockRow{scanFunc: func(dest ...any) error {
					return nil
				}}
			},
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
			queryRowFunc: func(ctx context.Context, sqlQuery string, arguments ...any) db.Row {
				return &mockRow{scanFunc: func(dest ...any) error {
					return sql.ErrNoRows
				}}
			},
			execFunc: func(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
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
