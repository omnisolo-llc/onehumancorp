package db

import (
	"context"
	"database/sql"

	_ "modernc.org/sqlite"
)

type TestingT interface {
	Helper()
	Fatalf(format string, args ...interface{})
	Cleanup(func())
}

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t TestingT) Provider {
	t.Helper()
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := db.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		db.Close()
	})

	return NewSqliteProvider(db)
}
