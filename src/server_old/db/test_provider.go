package db

import (
	"context"
	"database/sql"
	"fmt"
	"testing"

	"github.com/google/uuid"
	_ "modernc.org/sqlite"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t *testing.T) Provider {
	t.Helper()
	// Use a shared cache URI for :memory: to allow concurrent access across goroutines in the same process
	// since modernc.org/sqlite memory dbs disappear if the specific connection closes or if separate connections are opened
	// without a shared URI. We must generate a unique db name to avoid sharing state between separate tests.
	dbName := uuid.NewString()
	db, err := sql.Open("sqlite", fmt.Sprintf("file:%s?mode=memory&cache=shared", dbName))
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
