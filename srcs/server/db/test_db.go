package db

import (
	"database/sql"
	"testing"
	_ "modernc.org/sqlite"
)

func NewTestProvider(t *testing.T) Provider {
	t.Helper()
	db, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}
	return NewSqliteProvider(db)
}
