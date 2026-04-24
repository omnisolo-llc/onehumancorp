package lock

import (
	"context"
	"database/sql"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockPostgresDB struct {
	isSQLite bool
}

func (m *mockPostgresDB) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if sql == "DELETE FROM distributed_locks WHERE key = $1 AND token = $2" {
		return 1, nil
	}
	return 1, nil
}

func (m *mockPostgresDB) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	if sql == "SELECT 1" {
		return &mockPostgresRow{err: nil}
	}
	return &mockPostgresRow{err: nil, val: "key"}
}

func (m *mockPostgresDB) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, fmt.Errorf("not implemented")
}

func (m *mockPostgresDB) Begin(ctx context.Context) (db.Tx, error) {
	return nil, fmt.Errorf("not implemented")
}

func (m *mockPostgresDB) Close() {}

func (m *mockPostgresDB) Ping(ctx context.Context) error {
	return nil
}

func (m *mockPostgresDB) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockPostgresDB) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func (m *mockPostgresDB) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
	return nil, nil
}

type mockPostgresRow struct {
	err error
	val string
}

func (m *mockPostgresRow) Scan(dest ...any) error {
	if len(dest) > 0 {
		if v, ok := dest[0].(*string); ok && m.val != "" {
			*v = m.val
		}
	}
	return m.err
}

func TestDatabaseLockProvider_Postgres_Coverage(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockPostgresDB{isSQLite: false}
	provider := NewDatabaseLockProvider(mockDB)

	t.Run("acquire_new_lock", func(t *testing.T) {
		locked, unlock, err := provider.TryLock(ctx, "test-key-1", 10*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected to acquire lock")
		}
		if unlock == nil {
			t.Fatalf("Expected unlock function")
		}

		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on unlock, got %v", err)
		}
	})

	t.Run("acquire_failed_lock", func(t *testing.T) {
		// We need to override the row mock
		// for failure
		mockDB3 := &mockPostgresDBFailed{isSQLite: false}
		provider3 := NewDatabaseLockProvider(mockDB3)

		locked, unlock, err := provider3.TryLock(ctx, "test-key-2", 10*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if locked {
			t.Fatalf("Expected NOT to acquire lock")
		}
		if unlock != nil {
			t.Fatalf("Expected nil unlock function")
		}
	})
}

type mockPostgresDBFailed struct {
	isSQLite bool
}

func (m *mockPostgresDBFailed) Exec(ctx context.Context, query string, arguments ...any) (int64, error) {
	return 0, nil
}

func (m *mockPostgresDBFailed) QueryRow(ctx context.Context, query string, optionsAndArgs ...any) db.Row {
	return &mockPostgresRow{err: sql.ErrNoRows}
}

func (m *mockPostgresDBFailed) Query(ctx context.Context, query string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, fmt.Errorf("not implemented")
}

func (m *mockPostgresDBFailed) Begin(ctx context.Context) (db.Tx, error) {
	return nil, fmt.Errorf("not implemented")
}

func (m *mockPostgresDBFailed) Close() {}

func (m *mockPostgresDBFailed) Ping(ctx context.Context) error {
	return nil
}

func (m *mockPostgresDBFailed) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockPostgresDBFailed) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func (m *mockPostgresDBFailed) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
	return nil, nil
}
