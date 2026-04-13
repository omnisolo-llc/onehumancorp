package db

import (
	"database/sql"
	"fmt"
	_ "modernc.org/sqlite"
)

// NewSQLiteProvider is a helper method to instantiate a new sqlite provider with a given connection string (e.g. ":memory:")
func NewSQLiteProvider(dsn string) (Provider, error) {
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, fmt.Errorf("failed to open sqlite db: %w", err)
	}
	return NewSqliteProvider(db), nil
}
