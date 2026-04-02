package db

import (
	"database/sql"
	"fmt"
	_ "modernc.org/sqlite"
)

func NewSqliteProviderForTest(dbPath string) (Provider, error) {
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, fmt.Errorf("db: connect to sqlite: %w", err)
	}
	db.SetMaxOpenConns(1)
	return NewSqliteProvider(db), nil
}
