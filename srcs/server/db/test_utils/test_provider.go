package test_utils

import (
	"context"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// TestingT is an interface wrapper around *testing.T
type TestingT interface {
	Helper()
	Fatalf(format string, args ...any)
	Cleanup(f func())
}

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t TestingT) db.Provider {
	t.Helper()
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := d.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		d.Close()
	})

	return db.NewSqliteProvider(d)
}
