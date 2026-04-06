package db

import (
	"database/sql"
	"testing"

	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

// NewTestProvider initializes a SQLite test provider.
func NewTestProvider(t *testing.T) Provider {
	t.Helper()
	db, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	t.Cleanup(func() { db.Close() })

	return NewSqliteProvider(db)
}
