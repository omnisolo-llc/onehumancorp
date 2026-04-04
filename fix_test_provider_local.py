with open('srcs/server/orchestration/test_provider_test.go', 'w') as f:
    f.write("""package orchestration

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t *testing.T) db.Provider {
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
""")
