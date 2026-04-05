package dbtest

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	dbInstance, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := dbInstance.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		dbInstance.Close()
	})

	return db.NewSqliteProvider(dbInstance)
}
