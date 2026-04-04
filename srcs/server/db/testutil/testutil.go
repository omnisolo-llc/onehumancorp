package testutil

import (
	"context"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type testingTB interface {
	Helper()
	Fatalf(format string, args ...any)
	Cleanup(f func())
}

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t testingTB) db.Provider {
	t.Helper()
	dbInstance, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := dbInstance.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		dbInstance.Close()
	})

	return db.NewSqliteProvider(dbInstance)
}
