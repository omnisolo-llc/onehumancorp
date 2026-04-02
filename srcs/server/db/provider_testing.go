package db

import (
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

// NewSqliteProviderMemory is a helper for testing
func NewSqliteProviderMemory() (Provider, error) {
	db, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		return nil, err
	}
	db.SetMaxOpenConns(1)
	return NewSqliteProvider(db), nil
}

func TestDummy(t *testing.T) {
	// A dummy test to satisfy the go_test rule
}
