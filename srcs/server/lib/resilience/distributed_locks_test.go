package resilience

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDummyLock(t *testing.T) {
	lock := &DummyLock{}
	err := lock.Lock(context.Background(), time.Second)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	err = lock.Unlock(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestPostgresLock_SQLiteFallback(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	t.Cleanup(func() { sqlDB.Close() })
	prov := db.NewSqliteProvider(sqlDB)

	lock := NewPostgresLock(prov, "test_key")

	err = lock.Lock(context.Background(), time.Second)
	if err != nil {
		t.Fatalf("expected no error for SQLite fallback, got %v", err)
	}

	err = lock.Unlock(context.Background())
	if err != nil {
		t.Fatalf("expected no error for SQLite fallback, got %v", err)
	}
}
